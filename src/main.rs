use scanfs::scan_directory;
use scanfs::FsElementInfo;
use std::env;
mod scanfs;

const PATH_PARAMETER: usize = 1;

fn main() {
    let args: Vec<String> = env::args().collect();
    let found: Vec<FsElementInfo> = scan_directory(args.get(PATH_PARAMETER));

    for path in found {
        if path.level == 0 || path.level == 1 {
            println!("Map Entry [path: {:?} ", path);
        }
    }
}
