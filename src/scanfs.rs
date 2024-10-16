use std::collections::HashMap;
use std::fs::{self, DirEntry};

pub fn scan_directory(dir_path: Option<&String>) -> HashMap<String, DirEntry> {
    let mut fs_elements = HashMap::new();
    if let Some(path) = dir_path {
        let result_paths = fs::read_dir(path);
        if let Ok(present_paths) = result_paths {
            for result_one_path in present_paths {
                if let Ok(one_path_present) = result_one_path {
                    println!("Name: {}", one_path_present.path().display());
                    let owned_element: Option<String> = one_path_present.path().to_str().map(str::to_string);
                    if let Some(element_as_string) = owned_element {
                        fs_elements.insert(element_as_string, one_path_present);
                    }
                }
            }
        }
    }
    fs_elements
}
