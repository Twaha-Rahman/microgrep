use std::env;

use argument_parser;
use recursive_file_check;

fn main() {
    let arguments = argument_parser::parse().unwrap();

    println!("{:?}", arguments);

    let path_buffer = env::current_dir().unwrap();
    let current_path = path_buffer.to_str().unwrap();
    match recursive_file_check::search_files_in_folder(format!("{}/", current_path)) {
        Ok(output) => println!("{}", output),
        Err(reason) => eprintln!("Error occured! Here's the reason:\n{}", reason),
    }
}
