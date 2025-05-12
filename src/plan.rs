use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use crate::{collect_dishes, dish::Dish, types::IngredientList};

pub(crate) trait Plan {
    /// Generate a shopping list for all dishes.
    fn shopping_list(&self) -> IngredientList;
}

/// A single day with multiple dishes.
pub(crate) struct Day {
    /// List of dishes.
    pub(crate) dishes: Vec<Dish>,
    /// The amount of people to feed.
    pub(crate) people: Option<usize>,
}

impl Plan for Day {
    fn shopping_list(&self) -> IngredientList {
        todo!()
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
        todo!()
    }
}
const MEASURES: [&str; 11] = [
    "g", "mg", "kg", "el", "tl", "l", "ml", "Liter", "Scheiben", "scheiben", "scheibe",
];

/// Markdown meal plan loader.
/// Structure:
/// - String: (Num) People
/// - Strign: Start Date (chrono parsable string)
/// - String: ## Essensplan
/// - Markdown table of the form:
/// |                     | Donnerstag (14)                                                     | Freitag (14)               |
/// | ------------------- | ------------------------------------------------------------------- | -------------------------- |
/// | **Frühstück**       |                                                                     | [[Standartfrühstück]]      |
/// | **Mittagessen**     |                                                                     | [[Nudeln Mit Tomatensoße]] |
/// | **Kaffee & Kuchen** | #Einkauf                                                            | [[Blechkuchen]]            |
/// | **Abendessen**      | [[Maultaschen]]<br>[[Schmelzzwibeln]]<br>[[Kartoffel-Gurken-Salat]] | [[Curry]]                  |
/// Where:
/// - The first column is a list of eating times
/// - Then consecutive days follow where each day
/// - Has to have a heading that can be followed by number of people that participate
/// - Each row can contain
///   - multiple [[meals]] surrounded in double brackets
///   - A `#Einkauf` signal that you want to go shopping
struct MdPlanLoader {
    /// Path of the `essensplan.md` or similar file.
    file: PathBuf,
    /// Dir where all referenced meals can be found (recursively).
    dish_dir: Path,
}

impl MdPlanLoader {
    fn load(path: &Path, base_path: &Path) -> WeekPlan {
        let mut dishes = vec![];
        collect_dishes(&mut dishes, &base_path);
        let dishes = dishes
            .iter()
            .map(|path| (path.file_stem().unwrap().to_str().unwrap(), path))
            .collect::<HashMap<_, _>>();

        // Load the markdown plan file content
        let file_content =
            std::fs::read_to_string(path).expect("Failed to read markdown plan file");
        let mut lines = file_content.lines();
        // The first line is the number of people
        let people_line = lines.next().expect("Missing people count line");
        // The second line is the start date (expects format YYYY-MM-DD)
        let start_line = lines.next().expect("Missing start date line");
        let people = people_line
            .trim()
            .parse::<usize>()
            .expect("Failed to parse people count");
        let start = chrono::NaiveDate::parse_from_str(start_line.trim(), "%Y-%m-%d")
            .expect("Failed to parse start date");
        // TODO: Iterate table and collect days one after another
        let days = vec![];

        WeekPlan {
            start,
            days,
            people,
        }
    }
}

struct ListPlanLoader {}

impl ListPlanLoader {
    fn load(path: &Path) -> Self {
        todo!()
    }
}
