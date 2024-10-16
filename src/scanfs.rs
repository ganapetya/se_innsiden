use std::collections::HashMap;
use std::fs::{self, DirEntry};

pub fn scan_directory(dir_path: Option<&String>) -> HashMap<String, DirEntry> {
    let mut fs_elements = HashMap::new();
    //if we got an argumemnt
    if let Some(path) = dir_path {
        //read the dir
        let result_paths = fs::read_dir(path);
        //if no error while read
        if let Ok(present_paths) = result_paths {
            //iterate over internal dirs and files
            for result_one_path in present_paths {
                //if internal dir or path has been read without an error
                if let Ok(one_path_present) = result_one_path {
                    //print debug info
                    println!("Name: {}", one_path_present.path().display());
                    //convert Option<&str> to Option<String>
                    //means convert Option of ref to str to Option of owned String
                    //str - is immutable "view" 
                    //&str - is a read-only immutable ref into this view
                    //String - is in heap growable object
                    //let us question why I need String here at all..
                    let owned_element: Option<String> = one_path_present.path().to_str().map(str::to_string);
                    //if conversion was OK
                    if let Some(element_as_string) = owned_element {
                        //add this path element to the map
                        fs_elements.insert(element_as_string, one_path_present);
                    }
                }
            }
        }
    }
    fs_elements
}
