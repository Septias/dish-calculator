use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};

pub(crate) struct CookBook {
    dishes: HashMap<String, PathBuf>,
}

/// Collect all dishes recursively from the given path.
fn collect_dishes(dishes: &mut Vec<PathBuf>, path: &Path) {
    if path.is_dir() {
        let entries = fs::read_dir(path).unwrap().filter_map(Result::ok);
        entries.for_each(|entry| collect_dishes(dishes, &entry.path()))
    } else {
        dishes.push(path.to_path_buf());
    }
}

impl CookBook {
    pub(crate) fn from_file(path: &Path) -> Self {
        let mut dish_paths = vec![];
        collect_dishes(&mut dish_paths, path);

        let dishes = dish_paths
            .iter()
            .filter_map(|path| {
                path.file_stem()
                    .and_then(|stem| stem.to_str())
                    .map(|name| (name.to_string(), path.clone()))
            })
            .collect::<HashMap<_, _>>();

        Self { dishes }
    }

    /// Get a dish path by name.
    pub(crate) fn get(&self, name: &str) -> Option<&Path> {
        self.dishes.get(name).map(|p| p.as_path())
    }
}
