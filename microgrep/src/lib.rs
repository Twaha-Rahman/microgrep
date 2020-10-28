use colored::*;

use std::env;
use std::time::Instant;

mod minify_info;
mod time_taken;

use argument_parser::Agrguments;
use recursive_file_check;
use time_taken::time_taken_to_search;

pub fn run() -> Result<String, String> {
    let execution_start_time = Instant::now();

    let input = Agrguments::new(&mut env::args())?;

    match recursive_file_check::search_files_in_folder(&input) {
        Ok(value) => {
            let short_info = minify_info::minify_info(&value);

            print!("{}", short_info);

            for found_match in value {
                print!("{}", found_match);
            }
        }
        Err(reason) => {
            eprintln!(
                "{}",
                format!("\n {} {}", "Error occured:".red(), reason.to_string().red())
            );
        }
    }

    Ok(time_taken_to_search(execution_start_time))
}
