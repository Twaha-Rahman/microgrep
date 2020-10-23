//! # Recursive File Check
//! This libary is provides a simple functionalty of iterating
//! over all files under a directory.
//!
//! ## Example

use argument_parser::Agrguments;
use std::fs::{self, read_dir, DirEntry};
use std::io;
use std::path::Path;

mod dir_traverser;

pub fn search_files_in_folder(input_arguments: &Agrguments) -> Result<&'static str, io::Error> {
    dir_traverser::visit_dirs(input_arguments);
    Ok("placeholder_dir")
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
