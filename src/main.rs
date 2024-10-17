use scanfs::scan_directory;
use std::env;
//windows extention
use std::os::windows::fs::MetadataExt;
use std::fs;

mod scanfs;

const PATH_PARAMETER: usize = 1;

fn main() {
    let args: Vec<String> = env::args().collect();
    let found = scan_directory(args.get(PATH_PARAMETER));
    for path in found {
        let path_name = path.0;
        let metadata = fs::metadata(&path_name);
        if let Ok(metadata) = metadata {
            let file_size = metadata.file_size();
            println!("Map Entry [path: {:?} size: {:?}]", &path_name, file_size);
        }
    }
}
