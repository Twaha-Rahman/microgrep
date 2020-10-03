use std::env;
use std::process;

use colored::*;

pub struct UserInput {
    pub dir: String,
    pub search_string: String,
}

pub fn collect_env_vars(args: &mut env::Args) -> Result<(UserInput, &mut env::Args), &str> {
    let mut dir: String;
    let mut search_string: String = String::new();

    let mut error_occured = false;

    match args.next() {
        Some(value) => dir = format!("{}/", value),
        None => {
            eprintln!("ERROR: Unknown system error!");
            process::exit(1);
        }
    }

    match args.next() {
        Some(value) => dir = format!("{}{}", dir, value),
        None => {
            eprintln!("\n{}", "ERROR: PATH not provided!".red());
            error_occured = true;
        }
    }

    match args.next() {
        Some(value) => search_string = value,
        None => {
            eprintln!("\n{}", "ERROR: SEARCH_STRING not provided!".red());
            error_occured = true;
        }
    }

    if error_occured {
        return Err("Failed due to previous errors!");
    }

    Ok((UserInput { dir, search_string }, args))
}
