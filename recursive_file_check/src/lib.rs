//! # Recursive File Check
//! This libary is provides a simple functionalty of iterating
//! over all files under a directory.
//!
//! ## Example

use std::fs::read_dir;
use std::io;

pub fn search_files_in_folder(folder_dir: String) -> Result<String, io::Error> {
    Ok(folder_dir)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
