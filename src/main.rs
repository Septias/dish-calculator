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
use std::fs;

use crate::{cookbook::CookBook, plan::WeekPlan};

fn main() {
    let Cli { plan, dish_root } = Cli::parse();

    let cookbook = CookBook::from_file(&dish_root);
    let shopping_list = WeekPlan::from_file(&plan, &cookbook).shopping_list();

    let simple = shopping_list.as_md_list();
    fs::write("./shopping-list.md", &simple).expect("Failed to write shopping-list.md");
    let clustered = shopping_list.as_clustered_md_list();
    fs::write("./shopping-list-clustered.md", &clustered)
        .expect("Failed to write shopping-list-clustered.md");

    println!("Shopping lists generated successfully!");
}
