mod scanfs;
pub use scanfs::FsElementInfo;
use scanfs::scan_directory;
use scanfs::compare_file_with_info_by_level_size;
pub fn scan_directory_sort_by_size(dir_path: Option<&String>) -> Vec<FsElementInfo> {
    let mut elements = scan_directory(dir_path);
    elements.sort_by(compare_file_with_info_by_level_size);
    elements
}