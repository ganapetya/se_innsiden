mod scanfs;
use se_innsiden::scan_directory_sort_by_size;
use se_innsiden::FsElementInfo;
use std::env;

const PATH_PARAMETER: usize = 1;

fn main() {
    let args: Vec<String> = env::args().collect();
    let found: Vec<FsElementInfo> = scan_directory_sort_by_size(args.get(PATH_PARAMETER));
    for path in found {
        println!("Map Entry [path: {:?} ", path);
    }
}
