#![allow(unreachable_code)]
mod cli;
mod cookbook;
mod dish;
mod error;
mod plan;
mod types;

use clap::Parser;
use cli::Cli;
use error::DishPlanError;
use plan::Plan;
use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};

use crate::{cookbook::CookBook, plan::WeekPlan};

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

    let cookbook = CookBook::from_file(&dish_root);
    let plan = WeekPlan::from_file(&plan);

    let shopping_list = plan.shopping_list();
    let simple = shopping_list.as_md_list();
    fs::write("./shopping-list.md", &simple);
    let clustered = shopping_list.as_clustered_md_list();
    fs::write("./shopping-list-clustered.md", &clustered);
}
