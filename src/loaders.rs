use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use crate::{collect_dishes, types::WeekPlan};

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

        // TODO: Load start day
        let start = todo!();

        // TODO: Load number of people
        let people = todo!();
        // TODO: Iterate table and collect days one after another
        let days = vec![];

        WeekPlan {
            start,
            days,
            people,
        }
    }
}

struct DishLoader {
    /// Path of the `dish.md`
    file: Path,
}

impl DishLoader {
    fn load() {}
}

struct ListIngredientsLoader {}

struct TableIngrediensLoader {}
