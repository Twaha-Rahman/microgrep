use std::env;
use std::process;

use argument_parser;
use recursive_file_check;

pub fn run() {
    match argument_parser::parse() {
        Err(reason) => {
            eprintln!("{}", reason);
            process::exit(1)
        }
        Ok(value) => {
            println!("{:?}", value);

            let path_buffer = env::current_dir().unwrap();
            let current_path = path_buffer.to_str().unwrap();

            let current_directory = format!("{}/", current_path);

            match recursive_file_check::search_files_in_folder(current_directory) {
                Ok(output) => println!("{}", output),
                Err(reason) => eprintln!("Error occured! Here's the reason:\n{}", reason),
            }
        }
    }
}
