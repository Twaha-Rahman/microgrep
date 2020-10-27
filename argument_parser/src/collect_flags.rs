use colored::*;

pub fn collect_flags<T: Iterator<Item = String>>(args: &mut T) -> Vec<String> {
    let avalable_flags = ["--regex", "--case-sensitive", "--case-insensitive"];
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

#[cfg(test)]
mod tests {

    use crate::collect_flags;

    #[test]
    fn no_flags_passed() {
        let flags: [String; 0] = [];
        let mut args = flags.iter().map(|s| s.to_string());
        let parsed_env_vars = collect_flags(&mut args);
        assert_eq!(Vec::new() as Vec<String>, parsed_env_vars);
    }
    #[test]
    fn collects_single_flag() {
        let flags = ["--regex"];
        let mut args = flags.iter().map(|s| s.to_string());
        let parsed_env_vars = collect_flags(&mut args);
        assert_eq!(Vec::from(flags), parsed_env_vars);
    }

    #[test]
    fn collects_multiple_flags() {
        let flags = ["--regex", "--case-insensitive"];
        let mut args = flags.iter().map(|s| s.to_string());
        let parsed_env_vars = collect_flags(&mut args);

        assert_eq!(Vec::from(flags), parsed_env_vars);
    }

    #[test]
    fn filter_out_unknown_flags() {
        let flags = ["--regex", "--case-insensitive", "--invalid-flag"];
        let mut args = flags.iter().map(|s| s.to_string());
        let parsed_env_vars = collect_flags(&mut args);

        assert_eq!(
            Vec::from(["--regex", "--case-insensitive"]),
            parsed_env_vars
        );
    }
}
