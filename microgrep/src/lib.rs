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
        "\n{} {} found in {} {}.\n",
        total_match,
        if total_match == 1 { "match" } else { "matches" },
        match_found_in,
        if match_found_in == 1 { "file" } else { "files" }
    );

    formatted_info.yellow().to_string()
}

pub fn run() -> Result<(), String> {
    let input = argument_parser::parse(&mut env::args())?;

    println!("Debug Info: {:?}", input);

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
