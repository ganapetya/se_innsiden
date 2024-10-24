use scanfs::scan_directory_sort_by_size;
use scanfs::FsElementInfo;
use std::env;
mod scanfs;

const PATH_PARAMETER: usize = 1;

fn main() {
    let args: Vec<String> = env::args().collect();
    let found: Vec<FsElementInfo> = scan_directory_sort_by_size(args.get(PATH_PARAMETER));

    for path in found {
        println!("Map Entry [path: {:?} ", path);
    }
}
