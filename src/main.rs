#![allow(unreachable_code)]
mod cli;
pub mod loaders;
mod parser;
pub mod types;

use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};

use clap::Parser;
use cli::Cli;

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
    let Cli { plan, dish_root } = Cli::parse();

    let available_dishes = {
        let mut dishes = vec![];
        let base_path = dish_root.unwrap_or(PathBuf::from("./."));
        collect_dishes(&mut dishes, &base_path);
        dishes
            .iter()
            .map(|path| (path.file_stem().unwrap().to_str().unwrap(), path))
            .collect::<HashMap<_, _>>()
    };

    let wanted_dishes = todo!();
    let ingredients = wanted_dishes.scale();

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
