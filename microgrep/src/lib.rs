use std::env;
use std::error::Error;
use std::process;

use argument_parser;
use recursive_file_check;

pub fn run() -> Result<(), Box<dyn Error>> {
    let ans = argument_parser::parse(&mut env::args())?;

    println!("{:?}", ans);

    let path_buffer = env::current_dir()?;
    let current_path = path_buffer.to_str().unwrap_or_else(|| "");

    let current_directory = format!("{}/", current_path);

    let result = recursive_file_check::search_files_in_folder(current_directory)?;

    println!("{:?}", result);

    Ok(())
}
