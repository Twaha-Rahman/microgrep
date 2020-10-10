use std::env;

use argument_parser;
use recursive_file_check;

pub fn run() -> Result<(), String> {
    let input = argument_parser::parse(&mut env::args())?;

    println!("{:?}", input);

    match recursive_file_check::search_files_in_folder(&input) {
        Ok(value) => {
            println!("{:?}", value);
        }
        Err(reason) => {
            eprintln!("Error occured: {}", reason);
        }
    }

    Ok(())
}
