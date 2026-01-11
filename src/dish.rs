use std::path::PathBuf;

use tokio::fs;

use crate::types::Ingredient;
use thiserror::Error;

#[derive(Error, Debug)]
pub(crate) enum DishPlanError {
    #[error("The plan does not exist at the given location.")]
    DishError,
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
    async fn accumulate(&self) -> Vec<Ingredient> {
        /* let file = fs::read_to_string(&self.path).await.unwrap();
               let start = file.find("Zutaten").unwrap();
               let end = start + file[start..].find("#").unwrap_or(file.len() start);
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
        -
               let upscale = self.people / dish_participants as f32;
               bulletpoint_regex
                   .captures_iter(&file[start..end])
                   .map(|capture| {
                       let ingredient = capture.name("ingredient").unwrap().as_str().trim();
                       let parts = ingredient.split(' ').collect::<Vec<_>>();
                       if parts.len() == 0 {
                           panic!("no ingredient found")
                       }
        -
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
        -
                           let (measure, name) = ("".to_string(), parts[1..].join(" "));
        -
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
                   .collect() */
        todo!()
    }
}
