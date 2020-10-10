use std::fs;
use std::io;

use argument_parser::Agrguments;

use crate::file_match_seacher::search;

pub fn visit_dirs(input: &Agrguments) -> io::Result<Vec<Vec<String>>> {
    let Agrguments {
        dir,
        search_string,
        flags,
    } = input;

    let mut results = vec![];

    if dir.is_file() {
        let file_contents = fs::read_to_string(dir)?;
        let result = search(file_contents, search_string, flags);

        results.push(result);

        return Ok(results);
    }

    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(input)?;
            } else {
                let file_contents = fs::read_to_string(&path)?;

                let result = search(file_contents, search_string, flags);

                results.push(result);
            }
        }

        return Ok(results);
    }

    Ok(Vec::new())
}
