use std::env;
use std::path;

use collect_env_vars::{collect_env_vars, UserInput};
use collect_flags::collect_flags;
use colored::*;

mod collect_env_vars;
mod collect_flags;

#[derive(Debug)]
pub struct Agrguments {
    pub dir: path::PathBuf,
    pub search_string: String,
    pub flags: Vec<String>,
}

pub fn parse(args: &mut env::Args) -> Result<Agrguments, String> {
    let (UserInput { dir, search_string }, args_iterator) = collect_env_vars(args)?;

    let flags = collect_flags(args_iterator);
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
