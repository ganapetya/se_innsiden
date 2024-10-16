use std::collections::HashMap;
use std::fs::{self, DirEntry};

pub fn scan_directory(dir_path: Option<&String>) -> HashMap<String, DirEntry> {
    let mut fs_elements = HashMap::new();
    if let Some(path) = dir_path {
        let paths = fs::read_dir(path).unwrap();
        for one_path in paths {
            println!("Name: {}", one_path.as_ref().unwrap().path().display());
            let element = one_path.unwrap();
            let owned_element: Option<String> = element.path().to_str().map(str::to_string);
            if let Some(element_as_string) = owned_element {
                fs_elements.insert(element_as_string, element);
            }
        }
    }
    fs_elements
}
