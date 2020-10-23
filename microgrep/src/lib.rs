use colored::*;
use std::env;

use argument_parser;
use recursive_file_check;

fn minify_info(info: &Vec<recursive_file_check::MachesInFiles>) -> String {
    let match_found_in = info.len();

    let mut total_match = 0;

    for more_info in info {
        total_match += more_info.matches.len()
    }

    let formatted_info = format!(
        "\n{} match(es) found in {} file(s).\n",
        total_match, match_found_in
    );

    formatted_info.yellow().to_string()
}

pub fn run() -> Result<(), String> {
    let input = argument_parser::parse(&mut env::args())?;

    println!("{:?}", input);

    match recursive_file_check::search_files_in_folder(&input) {
        Ok(value) => {
            let short_info = minify_info(&value);

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
