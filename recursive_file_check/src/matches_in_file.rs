use std::fmt;

use colored::*;

pub struct MachesInFiles {
    pub filename: String,
    pub matches: Vec<(usize, String)>,
}

impl fmt::Display for MachesInFiles {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.matches.len() == 0 {
            return Ok(());
        }

        writeln!(
            f,
            "\n⚪ {} - ({} {})",
            self.filename.green(),
            self.matches.len(),
            if self.matches.len() > 1 {
                "matches"
            } else {
                "match"
            }
        )?;

        for (index, line_string) in &self.matches {
            writeln!(f, "\n  {} {}", format!("{} |", index).blue(), line_string)?;
        }

        Ok(())
    }
}
