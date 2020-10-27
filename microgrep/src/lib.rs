use std::env;

use argument_parser::Agrguments;
use recursive_file_check;

mod minify_info;

pub fn run() -> Result<(), String> {
    let input = Agrguments::new(&mut env::args())?;

    match recursive_file_check::search_files_in_folder(&input) {
        Ok(value) => {
            let short_info = minify_info::minify_info(&value);

            println!("{}", short_info);

            for found_match in value {
                println!("{}", found_match);
            }
        }
        Err(reason) => {
            eprintln!("Error occured: {}", reason);
        }
    }

    Ok(())
}
