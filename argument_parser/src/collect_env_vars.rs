use std::env;
use std::path;
use std::process;

use colored::*;

pub struct UserInput {
    pub dir: path::PathBuf,
    pub search_string: String,
}

pub fn collect_env_vars(args_iter: &mut env::Args) -> Result<(UserInput, &mut env::Args), &str> {
    let mut dir: String = String::new();
    let mut search_string: String = String::new();

    let mut error_occured = false;

    match args_iter.next() {
        Some(value) => println!("First agument in args: {}", value),
        None => {
            eprintln!("ERROR: Unknown system error!");
            process::exit(1);
        }
    }

    match args_iter.next() {
        Some(path) => dir = format!("{}", path),
        None => {
            eprintln!("\n{}", "ERROR: PATH not provided!".red());
            error_occured = true;
        }
    }

    match args_iter.next() {
        Some(value) => search_string = value,
        None => {
            eprintln!("\n{}", "ERROR: SEARCH_STRING not provided!".red());
            error_occured = true;
        }
    }

    let path_buf_of_dir = path::PathBuf::from(dir);

    if error_occured {
        return Err("Failed due to previous errors!");
    }

    Ok((
        UserInput {
            dir: path_buf_of_dir.to_path_buf(),
            search_string,
        },
        args_iter,
    ))
}
