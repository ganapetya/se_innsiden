use log::{info, warn};
use rayon::iter::ParallelBridge;
use rayon::prelude::ParallelIterator;
use std::fs::{self, ReadDir};
use std::os::linux::fs::MetadataExt;
use std::sync::mpsc::{channel, Sender};

#[derive(Debug)]
pub enum FsElementType {
    FILE,
    DIR,
}

#[derive(Debug)]
pub struct FsElementInfo {
    pub level: u32,
    path: String,
    size_bytes: u64,
    kind: FsElementType,
}

pub fn compare_file_with_info_by_level_size(a: &FsElementInfo, b: &FsElementInfo) -> std::cmp::Ordering {
    if a.level > b.level {
        return std::cmp::Ordering::Greater;
    }
    if a.level == b.level {
        if a.size_bytes < b.size_bytes {
            return std::cmp::Ordering::Greater;
        }
        if a.size_bytes == b.size_bytes {
            return std::cmp::Ordering::Equal;
        }
        return  std::cmp::Ordering::Less
    }
    std::cmp::Ordering::Less
}



pub fn scan_directory(dir_path: Option<&String>) -> Vec<FsElementInfo> {
    let receiver = {
        let (sender, receiver) = channel();

        scan_directory_req(dir_path, &sender, 0);

        receiver
    };
    receiver.into_iter().par_bridge().collect()
}

fn scan_directory_req(
    dir_path: Option<&String>,
    sender: &Sender<FsElementInfo>,
    level: u32,
) -> u64 {
    if let Some(path) = dir_path {
        let read_dir_result = fs::read_dir(path);
        return match read_dir_result {
            Ok(present_paths) => {
                info!("Collecting data from directory {}", path);
                collect_directory_data(present_paths, sender, level, path)
            }
            Err(error) => {
                warn!("Can not read directory {path} : {error}");
                0
            }
        };
    }
    0
}

fn collect_directory_data(
    present_paths: ReadDir,
    sender: &Sender<FsElementInfo>,
    level: u32,
    parent_path: &String,
) -> u64 {
    let mut result: u64 = 0;

    present_paths.for_each(|result_one_path| {
        if let Ok(one_path_present) = result_one_path {
            let path_name = one_path_present.path();
            if let Ok(file_type) = one_path_present.file_type() {
                if file_type.is_dir() {
                    let path_param: Option<String> =
                        one_path_present.path().to_str().map(|e| e.to_string());
                    result += scan_directory_req(path_param.as_ref(), sender, level + 1);
                } else if file_type.is_file() {
                    let metadata = fs::metadata(&path_name);
                    if let Ok(metadata) = metadata {
                        let file_size = metadata.st_size();
                        result += file_size;
                        let file_info = FsElementInfo {
                            level: level + 1,
                            path: path_name.display().to_string(),
                            size_bytes: file_size,
                            kind: FsElementType::FILE,
                        };
                        let _ = sender.send(file_info);
                    }
                }
            }
        }
    });

    let file_info = FsElementInfo {
        level,
        path: parent_path.to_string(),
        size_bytes: result,
        kind: FsElementType::DIR,
    };
    let _ = sender.send(file_info);

    result
}
