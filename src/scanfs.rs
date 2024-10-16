use rayon::iter::ParallelBridge;
use rayon::prelude::ParallelIterator;
use std::collections::HashMap;
use std::fs::{self, DirEntry, ReadDir};
use std::sync::mpsc::{channel, Sender};

pub fn scan_directory(dir_path: Option<&String>) -> HashMap<String, DirEntry> {
    let reciever = {
        let (sender, reciever) = channel();

        scan_directory_send(dir_path, &sender);

        reciever
    };

    let all_paths: Vec<DirEntry> = reciever.into_iter().par_bridge().collect();

    let mut fs_elements = HashMap::new();

    for file_system_element in all_paths {
        let owned_element: Option<String> = file_system_element.path().to_str().map(str::to_string);
        if let Some(element_as_string) = owned_element {
            fs_elements.insert(element_as_string, file_system_element);
        }
    }

    fs_elements
}

pub fn scan_directory_send(dir_path: Option<&String>, sender: &Sender<DirEntry>) -> () {
    if let Some(path) = dir_path {
        let result_paths = fs::read_dir(path);
        if let Ok(present_paths) = result_paths {
            collect_fs_data(present_paths, sender);
        }
    }
}

fn collect_fs_data(present_paths: ReadDir, sender: &Sender<DirEntry>) -> () {
    present_paths.for_each(|result_one_path| {
        if let Ok(one_path_present) = result_one_path {
            if let Ok(file_type) = one_path_present.file_type() {
                if file_type.is_dir() {
                    let path_param: Option<String> =
                        one_path_present.path().to_str().map(|e| e.to_string());
                    scan_directory_send(path_param.as_ref(), sender);
                }
                //TODO: process or note negative result
                let _ = sender.send(one_path_present);
            }
        }
    });
}
