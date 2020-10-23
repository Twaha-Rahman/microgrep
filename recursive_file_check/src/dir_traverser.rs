use std::fmt;
use std::fs;
use std::io;

use colored::*;

use argument_parser::Agrguments;

#[derive(Debug)]
pub struct MachesInFiles {
    filename: String,
    pub matches: Vec<(usize, String)>,
}

impl fmt::Display for MachesInFiles {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut printable_lines: Vec<String> = Vec::new();

        writeln!(
            f,
            "⚪ {} - ({} match(es))",
            self.filename.green(),
            self.matches.len()
        );

        for (index, line_string) in &self.matches {
            writeln!(f, "\n  {} {}", format!("{} |", index).blue(), line_string);
        }

        Ok(())
    }
}

pub fn visit_dirs(input: &Agrguments) -> io::Result<Vec<MachesInFiles>> {
    let search = |text_to_search: String| {
        let mut matched_lines = vec![];
        for (index, line_string) in text_to_search.lines().enumerate() {
            let line_string = line_string.to_owned();

            if line_string.contains(&input.search_string) {
                matched_lines.push((index, line_string));
            }
        }
        matched_lines
    };

    let mut results = vec![];

    if input.dir.is_file() {
        let file_contents = fs::read_to_string(&input.dir)?;
        let result = search(file_contents);

        let filename = input.dir.to_str().unwrap().to_owned();
        let search_matches = MachesInFiles {
            filename,
            matches: result,
        };
        results.push(search_matches);

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

                let result = search(file_contents);

                let filename = path.to_str().unwrap().to_owned();
                let search_matches = MachesInFiles {
                    filename,
                    matches: result,
                };
                results.push(search_matches);
            }
        }

        return Ok(results);
    }

    Ok(Vec::new())
}
