use scanfs::scan_directory;
use std::env;

mod scanfs;

const PATH_PARAMETER: usize = 1;

fn main() {
    let args: Vec<String> = env::args().collect();
    let found = scan_directory(args.get(PATH_PARAMETER));

    for path in found {
        println!("Map Entry [path: {:?} ", path);
    }
}
