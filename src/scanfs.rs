use rayon::iter::ParallelBridge;
use rayon::prelude::ParallelIterator;
use std::collections::HashMap;
use std::fs::{self, DirEntry, ReadDir};
use std::sync::mpsc::{channel, Sender};
use log::{info, warn};

pub fn scan_directory(dir_path: Option<&String>) -> HashMap<String, DirEntry> {
    let reciever = {
        let (sender, reciever) = channel();

        scan_directory_req(dir_path, &sender);

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

fn scan_directory_req(dir_path: Option<&String>, sender: &Sender<DirEntry>) -> () {
    if let Some(path) = dir_path {
        let read_dir_result = fs::read_dir(path);
        match read_dir_result {
            Ok(present_paths) => {
                info!("Collecting data from directory {}", path);
                collect_directory_data(present_paths, sender)
            },
            Err(error) => warn!("Can not read directory {path} : {error}"),
        };
    }
}

fn collect_directory_data(present_paths: ReadDir, sender: &Sender<DirEntry>) -> () {
    present_paths.for_each(|result_one_path| {
        if let Ok(one_path_present) = result_one_path {
            let path_name = one_path_present.path();
            if let Ok(file_type) = one_path_present.file_type() {
                if file_type.is_dir() {
                    let path_param: Option<String> =
                        one_path_present.path().to_str().map(|e| e.to_string());
                    scan_directory_req(path_param.as_ref(), sender);
                }
                match sender.send(one_path_present) {
                    Ok(_) => {
                        info!("Successfully sent into the queue file info {:?}", &path_name);            
                    },
                    Err(error) => warn!("Can not send into queue fine info {:?} : {error}", &path_name),
                }
            }
        }
    });
}
