use std::env;

struct Agrguments<'a> {
    dir: String,
    search_string: String,
    flags: Vec<&'a str>,
}

pub fn parse() {
    let avalable_flags = ["regex", "case-sensitive", "case-insensitive"];
    let args: Vec<String> = env::args().collect();

    let mut flags: Vec<&str> = vec![];

    for argument in args.iter() {
        if argument.starts_with("-") {
            let flag = &argument[1..];
            if avalable_flags.contains(&flag) {
                flags.push(flag);
            } else {
                println!("WARN: Unknown flag `{}` provided.", flag);
            }
        }
    }

    println!("{}", args.join(" "));
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
