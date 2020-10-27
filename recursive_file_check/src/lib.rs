//! # Recursive File Check
//! This libary is provides a simple functionalty of iterating
//! over all files under a directory.
//!
//! ## Example

use argument_parser::Agrguments;
pub use dir_traverser::MachesInFiles;
use std::io;

mod dir_traverser;
mod matches_in_file;

pub fn search_files_in_folder(input_arguments: &Agrguments) -> io::Result<Vec<MachesInFiles>> {
    dir_traverser::visit_dirs(input_arguments)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
