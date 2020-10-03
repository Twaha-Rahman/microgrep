use std::env;

use colored::*;

pub fn collect_flags(args: &mut env::Args) -> Vec<String> {
    let avalable_flags = ["-regex", "-case-sensitive", "-case-insensitive"];
    let mut flags: Vec<String> = vec![];

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

    flags
}
