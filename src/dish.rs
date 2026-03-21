use std::{fs, path::{Path, PathBuf}};

use anyhow::{Context, Result};
use regex::Regex;

use crate::types::Ingredient;
use thiserror::Error;

const MEASURES: [&str; 11] = [
    "g", "mg", "kg", "el", "tl", "l", "ml", "Liter", "Scheiben", "scheiben", "scheibe",
];

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
    pub(crate) fn from_file(path: &Path, dish_name: &str, people: usize) -> Result<Self> {
        let content = fs::read_to_string(path)
            .with_context(|| format!("Failed to read dish file: {}", path.display()))?;

        // Extract recipe portion count
        let portions_regex = Regex::new(r"(?<portions>\d+)\s*(?:Persons|Portionen|Personen)")
            .context("Failed to compile portions regex")?;

        let recipe_people = portions_regex
            .captures(&content)
            .and_then(|cap| cap.name("portions"))
            .and_then(|m| m.as_str().parse::<usize>().ok())
            .unwrap_or(1);

        // Find Zutaten section
        let zutaten_start = content
            .find("## Zutaten")
            .or_else(|| content.find("##Zutaten"))
            .context("No '## Zutaten' section found")?;

        let zutaten_section_start = zutaten_start + "## Zutaten".len();
        let zutaten_end = content[zutaten_section_start..]
            .find("\n##")
            .map(|pos| zutaten_section_start + pos)
            .unwrap_or(content.len());

        let zutaten_text = &content[zutaten_section_start..zutaten_end];

        // Parse ingredient lines
        let bulletpoint_regex = Regex::new(r"-\s*(?<ingredient>.+)")
            .context("Failed to compile bulletpoint regex")?;

        let scale_factor = people as f32 / recipe_people as f32;

        let ingredients: Vec<Ingredient> = bulletpoint_regex
            .captures_iter(zutaten_text)
            .filter_map(|cap| {
                let ingredient_str = cap.name("ingredient")?.as_str().trim();
                parse_ingredient_line(ingredient_str, dish_name, scale_factor)
            })
            .collect();

        Ok(Self {
            people: Some(people),
            recepie_people: recipe_people,
            ingredients,
            blocks: vec![],
            path: path.to_path_buf(),
        })
    }
}

fn parse_ingredient_line(line: &str, dish_name: &str, scale_factor: f32) -> Option<Ingredient> {
    let parts: Vec<&str> = line.split_whitespace().collect();

    if parts.is_empty() {
        return None;
    }

    if parts.len() == 1 {
        // Just a name, no amount
        return Some(Ingredient {
            amount: scale_factor,
            measure: String::new(),
            name: parts[0].to_string(),
            dish: dish_name.to_string(),
        });
    }

    // Try to parse first token as amount
    let (amount, measure_start) = match parts[0].parse::<f32>() {
        Ok(amt) => (amt * scale_factor, 1),
        Err(_) => {
            // First token is not a number, treat as name
            return Some(Ingredient {
                amount: scale_factor,
                measure: String::new(),
                name: parts.join(" "),
                dish: dish_name.to_string(),
            });
        }
    };

    // Check if second token is a unit
    let (measure, name_start) = if measure_start < parts.len()
        && MEASURES.contains(&parts[measure_start])
    {
        (parts[measure_start].to_string(), measure_start + 1)
    } else {
        (String::new(), measure_start)
    };

    let name = parts[name_start..].join(" ");

    Some(Ingredient {
        amount,
        measure,
        name,
        dish: dish_name.to_string(),
    })
}
