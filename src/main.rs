#![allow(unreachable_code)]
mod cli;
mod dish;
mod loaders;
mod plan;
mod types;

use clap::Parser;
use cli::Cli;
use plan::Plan;
use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};

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

    let plan = fs::read_to_string(plan.unwrap_or(PathBuf::from("./plan.md")));
    let wanted_dishes: Box<dyn Plan> = todo!();
    let ingredients = wanted_dishes.shopping_list();
    let simple = ingredients.as_md_list();
    fs::write("./list.md", &simple);
    let clustered = ingredients.as_clustered_md_list();
    fs::write("./list_clustered.md", &clustered);
}
