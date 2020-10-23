use colored::*;

pub fn minify_info(info: &Vec<recursive_file_check::MachesInFiles>) -> String {
    let match_found_in = info.len();

    let mut total_match = 0;

    for more_info in info {
        total_match += more_info.matches.len()
    }

    let formatted_info = format!(
        "\n{} {} found in {} {}.\n",
        total_match,
        if total_match == 1 { "match" } else { "matches" },
        match_found_in,
        if match_found_in == 1 { "file" } else { "files" }
    );

    formatted_info.yellow().to_string()
}
