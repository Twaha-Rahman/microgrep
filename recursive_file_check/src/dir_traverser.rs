use std::fs;
use std::io;

use argument_parser::Agrguments;

use crate::file_match_seacher::search;

pub fn visit_dirs(input: &Agrguments) -> io::Result<Vec<Vec<String>>> {
    let search_closure = |text_to_search: String| {
        let mut matched_lines = vec![];
        for line in text_to_search.lines().map(String::from) {
            if line.contains(&input.search_string) {
                matched_lines.push(line);
            }
        }
        println!("{:?}", matched_lines);
        matched_lines
    };

    let mut results = vec![];

    if input.dir.is_file() {
        let file_contents = fs::read_to_string(&input.dir)?;
        let result = search_closure(file_contents);

        results.push(result);

        return Ok(results);
    }

    if input.dir.is_dir() {
        for entry in fs::read_dir(&input.dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(input)?;
            } else {
                let file_contents = fs::read_to_string(&path)?;

                let result = search_closure(file_contents);

                results.push(result);
            }
        }

        return Ok(results);
    }

    Ok(Vec::new())
}
