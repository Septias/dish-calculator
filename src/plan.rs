use std::{fs, iter::Sum, ops::Add, path::Path};

use anyhow::Context;
use tree_sitter::Parser;

use crate::{cookbook::CookBook, dish::Dish, types::IngredientList, DishPlanError};

pub(crate) trait Plan {
    /// Generate a shopping list for all dishes.
    fn shopping_list(&self) -> IngredientList;
    fn from_file(path: &Path, cookbook: &CookBook) -> Self;
}

/// A single day with multiple dishes.
pub(crate) struct Day {
    /// List of dishes.
    pub(crate) dishes: Vec<Dish>,
}

impl Plan for Day {
    fn shopping_list(&self) -> IngredientList {
        self.dishes
            .iter()
            .map(|dish| IngredientList::from(dish.ingredients.clone()))
            .sum()
    }

    fn from_file(_path: &Path, _cookbook: &CookBook) -> Self {
        // This is not used - we use WeekPlan::from_file instead
        unimplemented!("Use WeekPlan::from_file for parsing meal plans")
    }
}

impl Add for IngredientList {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        self.0.extend(rhs.0);
        self
    }
}

impl Sum for IngredientList {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(IngredientList::new(), |acc, list| acc + list)
    }
}

/// The week structure of a meal plan.
pub(crate) struct WeekPlan {
    /// Start date used for PDF export.
    pub(crate) start: chrono::NaiveDate,
    /// Consecutive list of days.
    pub(crate) days: Vec<Day>,
    /// The amount of people to feed.
    pub(crate) people: usize,
}

impl Plan for WeekPlan {
    fn shopping_list(&self) -> IngredientList {
        self.days.iter().map(|day| day.shopping_list()).sum()
    }

    fn from_file(path: &Path, cookbook: &CookBook) -> Self {
        let content = fs::read_to_string(path)
            .with_context(|| format!("Failed to read plan file: {}", path.display()))
            .unwrap();

        let mut parser = Parser::new();
        parser
            .set_language(&tree_sitter_menu::LANGUAGE.into())
            .expect("Error loading menu parser");

        let tree = parser.parse(&content, None).expect("Failed to parse");
        let root = tree.root_node();

        eprintln!("Root node kind: {}", root.kind());
        eprintln!("Root has error: {}", root.has_error());
        eprintln!("Tree: {}", root.to_sexp());

        let mut cursor = root.walk();

        let mut people = 1;
        let mut start_date = chrono::NaiveDate::from_ymd_opt(2026, 1, 1).unwrap();
        let mut days = Vec::new();

        for child in root.children(&mut cursor) {
            eprintln!("Child kind: {}", child.kind());
            match child.kind() {
                "persons_line" => {
                    if let Some(count_node) = child.child_by_field_name("count") {
                        let count_str = content[count_node.byte_range()].trim();
                        people = count_str.parse().unwrap_or(1);
                        eprintln!("Parsed people: {}", people);
                    }
                }
                "starttag_line" => {
                    if let Some(date_node) = child.child_by_field_name("date") {
                        let date_str = content[date_node.byte_range()].trim();
                        start_date = chrono::NaiveDate::parse_from_str(date_str, "%Y-%m-%d")
                            .unwrap_or_else(|_| {
                                chrono::NaiveDate::from_ymd_opt(2026, 1, 1).unwrap()
                            });
                        eprintln!("Parsed date: {}", start_date);
                    }
                }
                "day_line" => {
                    let day = parse_day_line(&child, &content, cookbook, people);
                    eprintln!("Parsed day with {} dishes", day.dishes.len());
                    days.push(day);
                }
                _ => {}
            }
        }

        eprintln!("Total days: {}", days.len());
        eprintln!(
            "Total dishes across all days: {}",
            days.iter().map(|d| d.dishes.len()).sum::<usize>()
        );

        Self {
            start: start_date,
            days,
            people,
        }
    }
}

fn parse_day_line(
    node: &tree_sitter::Node,
    content: &str,
    cookbook: &CookBook,
    default_people: usize,
) -> Day {
    let mut day_people = None;
    let mut dishes = Vec::new();

    let mut cursor = node.walk();

    eprintln!("  Day line children:");
    for child in node.children(&mut cursor) {
        eprintln!("    - kind: {}", child.kind());
        match child.kind() {
            "day_with_count" => {
                if let Some(count_node) = child.child_by_field_name("count") {
                    let count_str = content[count_node.byte_range()].trim();
                    day_people = count_str.parse().ok();
                }
            }
            "menu" => {
                eprintln!("    Found menu node");
                parse_menu(
                    &child,
                    content,
                    cookbook,
                    &mut dishes,
                    default_people,
                    day_people,
                );
            }
            _ => {}
        }
    }

    Day { dishes }
}

fn parse_menu(
    node: &tree_sitter::Node,
    content: &str,
    cookbook: &CookBook,
    dishes: &mut Vec<Dish>,
    default_people: usize,
    day_people: Option<usize>,
) {
    let mut cursor = node.walk();

    eprintln!("      Menu children:");
    for child in node.children(&mut cursor) {
        eprintln!("        - kind: {}", child.kind());
        match child.kind() {
            "rest_day" => {
                eprintln!("        Rest day - skipping");
                // Skip rest days
                return;
            }
            "menu_items" => {
                eprintln!("        Found menu_items");
                let mut items_cursor = child.walk();
                for item in child.children(&mut items_cursor) {
                    eprintln!("          Item kind: {}", item.kind());
                    if item.kind() == "menu_item" {
                        parse_menu_item(
                            &item,
                            content,
                            cookbook,
                            dishes,
                            default_people,
                            day_people,
                        );
                    }
                }
            }
            _ => {}
        }
    }
}

fn parse_menu_item(
    node: &tree_sitter::Node,
    content: &str,
    cookbook: &CookBook,
    dishes: &mut Vec<Dish>,
    default_people: usize,
    day_people: Option<usize>,
) {
    // The menu_item node directly contains either dish_with_count or shopping_marker
    // Get the first child which should be the actual content
    // let mut _cursor = node.walk();

    if let Some(child) = node.child(0) {
        eprintln!("          Menu item child kind: {}", child.kind());
        match child.kind() {
            "dish_with_count" => {
                if let Some(dish_node) = child.child_by_field_name("dish") {
                    eprintln!("            Found dish node: {}", dish_node.kind());
                    eprintln!(
                        "            Dish node children: {:?}",
                        (0..dish_node.child_count())
                            .map(|i| dish_node.child(i).unwrap().kind())
                            .collect::<Vec<_>>()
                    );
                    // The dish node has a nested "name" field
                    if let Some(name_node) = dish_node.child_by_field_name("name") {
                        let dish_name = content[name_node.byte_range()].trim();
                        eprintln!("            Dish name: {}", dish_name);

                        // Extract multiplier if present
                        let dish_people = child.child_by_field_name("count").map(|count_node| {
                            let count_str = content[count_node.byte_range()].trim();
                            // Remove parentheses from count
                            let count_str = count_str.trim_start_matches('(').trim_end_matches(')');
                            count_str.parse::<usize>().unwrap_or(1)
                        });

                        // Look up dish in cookbook
                        if let Some(dish_path) = cookbook.get(dish_name) {
                            eprintln!("            Found in cookbook: {:?}", dish_path);

                            // use the proper amount of people!
                            let people =
                                dish_people.unwrap_or(day_people.unwrap_or(default_people));
                            match Dish::from_file(dish_path, dish_name, people) {
                                Ok(dish) => {
                                    eprintln!(
                                        "            Loaded dish with {} ingredients",
                                        dish.ingredients.len()
                                    );
                                    dishes.push(dish);
                                }
                                Err(e) => {
                                    eprintln!("            Error loading dish: {}", e);
                                }
                            }
                        } else {
                            eprintln!("            NOT found in cookbook");
                        }
                    } else {
                        eprintln!("            No name node found");
                    }
                } else {
                    eprintln!("            No dish node found");
                }
            }
            "shopping_marker" => {
                eprintln!("            Found shopping marker");
                // TODO: Handle shopping markers in the future
            }
            _ => {}
        }
    }
}
