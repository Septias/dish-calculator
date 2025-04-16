pub mod loaders;
mod parser;
pub mod types;

use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};

use regex::Regex;

/// Calculate the ingredients needed for the number of participants and dish.

/// Collect all dishes recursively from the given path.
fn collect_dishes(dishes: &mut Vec<PathBuf>, path: &Path) {
    if path.is_dir() {
        let entries = fs::read_dir(path).unwrap().filter_map(Result::ok);
        entries.for_each(|entry| collect_dishes(dishes, &entry.path()))
    } else {
        dishes.push(path.to_path_buf());
    }
}

fn main() {
    let mut dishes = vec![];
    let base_path = Path::new(BASE);
    collect_dishes(&mut dishes, &base_path);
    let dishes = dishes
        .iter()
        .map(|path| (path.file_stem().unwrap().to_str().unwrap(), path))
        .collect::<HashMap<_, _>>();

    // Extract needed dishes from a dish plan
    // Format: [[<dish>]](<amount>)
    let regex = Regex::new(r#"\[\[(?<name>.*?)\]\](\((?<num>\d+)\))?"#).unwrap();
    let items = regex.captures_iter(PLAN).map(|c| {
        (
            c.name("name").unwrap().as_str(),
            c.name("num").map(|a| a.as_str().parse::<f32>().unwrap()),
        )
    });

    // Calculate amount of ingredients for each dish and how many participants are planned
    let calcs = items.map(|(name, amount)| {
        println!("Upscaling {} with {}", name, amount.unwrap_or(PARTICIPANTS));
        let calulation = calculate(
            dishes
                .get(name)
                .expect(&format!("dish {name} not in dishes"))
                .as_path(),
            name,
            amount.unwrap_or(PARTICIPANTS),
        );
        calulation
    });

    // Add all the amounts
    let accumulated_amounts = calcs
        .map(|calc| {
            calc.into_iter()
                .map(|amount| (amount.name.clone(), amount))
                .collect::<HashMap<_, _>>()
        })
        .fold(
            HashMap::new(),
            |mut acc: HashMap<String, Vec<Amount>>, elem| {
                elem.into_iter().for_each(|(name, amount)| {
                    if let Some(amounts) = acc.get_mut(&name) {
                        amounts.push(amount)
                    } else {
                        acc.insert(name, vec![amount]);
                    }
                });
                acc
            },
        );

    // Print the ingredient list
    let mut all = vec![];
    for (name, amount) in accumulated_amounts {
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
    fs::write("ingredients.md", all.join("\n")).unwrap();
}
