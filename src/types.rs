use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};

type Ingredients = Vec<Ingredient>;
struct IngredientList(Ingredients);

impl IngredientList {
    /// Accumulate ingredients.
    fn accumulate(&mut self) {
        self.0 = self
            .0
            .into_iter()
            .fold(
                HashMap::new(),
                |mut acc: HashMap<String, Vec<Ingredient>>, elem| {
                    if let Some(amounts) = acc.get_mut(&elem.name) {
                        amounts.push(elem)
                    } else {
                        acc.insert(elem.name, vec![elem]);
                    }

                    acc
                },
            )
            .iter()
            .collect()
    }
    /// Generate md shopping list.
    pub(crate) fn as_md_list(&self) -> String {
        let mut all = vec![];

        for (name, amount) in self.0 {
            all.push(format!(
                "- {name} [{}] ({})",
                amount
                    .iter()
                    .map(|amount| format!("{}{}", amount.amount, amount.measure))
                    .collect::<Vec<_>>()
                    .join(", "),
                amount
                    .iter()
                    .map(|amount| amount.dish.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
        }
        all.sort();
        all.join("\n")
    }

    /// Generate clustered md shopping list with AI.
    pub(crate) fn as_clustered_md_list(&self) -> String {
        todo!()
    }
}

pub(crate) trait Plan {
    /// Generate a shopping list for all dishes.
    fn shopping_list(&self) -> IngredientList;
}

/// A single ingredient
pub(crate) struct Ingredient {
    /// Amount of ingredient.
    pub(crate) amount: f32,
    /// Measure of the ingredient.
    pub(crate) measure: String,
    /// Name of the ingredient.
    pub(crate) name: String,
    /// The dish this ingredient is from.
    pub(crate) dish: String,
}

/// A single dish.
pub(crate) struct Dish {
    /// The amount of people to feed.
    pub(crate) people: Option<usize>,
    /// For how many people the recipe is scaled.
    pub(crate) recepie_people: usize,
    /// List of recipe ingredients.
    pub(crate) ingredients: Vec<Ingredient>,
    /// Other text
    pub(crate) blocks: Vec<String>,
    /// Path
    pub(crate) path: PathBuf,
}

impl Dish {
    fn accumulate(&self) -> Vec<Ingredient> {
        let file = fs::read_to_string(&self.path).unwrap();
        let start = file.find("Zutaten").unwrap();
        let end = start + file[start..].find("#").unwrap_or(file.len() - start);

        let bulletpoint_regex = Regex::new("-(?<ingredient>.+)").unwrap();
        let participants_regex =
            Regex::new(r"(?<participants>\d+) ?(Persons|Portionen|Personen)").unwrap();
        let dish_participants = participants_regex
            .captures(&file)
            .expect(format!("Persons have to be given for {}", self.path.display()).as_str())
            .name("participants")
            .unwrap()
            .as_str()
            .parse::<usize>()
            .expect("Can't pares participants");

        let upscale = self.people / dish_participants as f32;
        bulletpoint_regex
            .captures_iter(&file[start..end])
            .map(|capture| {
                let ingredient = capture.name("ingredient").unwrap().as_str().trim();
                let parts = ingredient.split(' ').collect::<Vec<_>>();
                if parts.len() == 0 {
                    panic!("no ingredient found")
                }

                if parts.len() == 1 {
                    Ingredient {
                        amount: 1.0,
                        measure: "".to_string(),
                        name: parts[0].to_string(),
                        dish: dish.to_string(),
                    }
                } else {
                    let amount = match parts[0].parse::<f32>() {
                        Ok(amount) => amount,
                        Err(_) => {
                            println!("couldn't parse unit {}, defaulting to 0", parts[0]);
                            0.0
                        }
                    };

                    let (measure, name) = ("".to_string(), parts[1..].join(" "));

                    Ingredient {
                        amount,
                        measure,
                        name,
                        dish: dish.to_string(),
                    }
                }
            })
            // upscale the amounts
            .map(|amount| Ingredient {
                amount: amount.amount * upscale,
                ..amount
            })
            .collect()
    }
}

/// A single day with multiple dishes.
pub(crate) struct Day {
    /// List of dishes.
    pub(crate) dishes: Vec<Dish>,
    /// The amount of people to feed.
    pub(crate) people: Option<usize>,
}

impl Plan for Day {
    fn shopping_list(&self) -> Ingredients {
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
    fn shopping_list(&self) -> Ingredients {
        todo!()
    }
}
