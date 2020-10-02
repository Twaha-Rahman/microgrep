use std::env;
use std::process;

use colored::*;

#[derive(Debug)]
pub struct Agrguments {
    dir: String,
    search_string: String,
    flags: Vec<String>,
}

pub fn parse() -> Result<Agrguments, String> {
    let avalable_flags = ["-regex", "-case-sensitive", "-case-insensitive"];
    let mut flags: Vec<String> = vec![];
    let mut args = env::args();

    let mut dir: String = String::new();
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

    if error_occured == true {
        return Err(String::from("\nFailed due to previous error!"));
    }

    let args: Vec<String> = args.collect();

    for argument in args.into_iter() {
        if argument.starts_with("-") {
            let flag = argument;
            if avalable_flags.contains(&flag.as_str()) {
                flags.push(flag);
            } else {
                let warning_msg = format!("WARN: Unknown flag `{}` provided.", flag);
                println!("\n{}", warning_msg.yellow());
            }
        }
    }
    return Ok(Agrguments {
        dir,
        search_string,
        flags,
    });
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
