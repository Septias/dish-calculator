use std::{
    fs,
    path::{Path, PathBuf},
};

pub(crate) struct CookBook {}

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
        let available_dishes = {
            let mut dishes = vec![];
            let base_path = dish_root.unwrap_or(PathBuf::from("./."));
            collect_dishes(&mut dishes, &base_path);
            dishes
                .iter()
                .map(|path| (path.file_stem().unwrap().to_str().unwrap(), path))
                .collect::<HashMap<_, _>>()
        };
    }
}
