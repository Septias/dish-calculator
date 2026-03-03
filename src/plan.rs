use std::{fs, iter::Sum, ops::Add, path::Path};

const MEASURES: [&str; 11] = [
    "g", "mg", "kg", "el", "tl", "l", "ml", "Liter", "Scheiben", "scheiben", "scheibe",
];

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
    /// The amount of people to feed.
    pub(crate) people: Option<usize>,
}

impl Plan for Day {
    fn shopping_list(&self) -> IngredientList {
        todo!()
    }

    fn from_file(path: &Path, cookbook: &CookBook) -> Self {
        let file = fs::read_to_string(path)
            .map_err(|_e| DishPlanError::PlanDoesNotExist)
            .unwrap();
        todo!()
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
        self.days.iter().map(|day| day.shopping_list()).sum()
    }

    fn from_file(path: &Path, cookbook: &CookBook) -> Self {
        todo!()
    }
}

struct ListPlanLoader {}

impl ListPlanLoader {
    fn load(path: &Path) -> Self {
        todo!()
    }
}
