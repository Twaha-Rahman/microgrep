pub fn search(text_to_search: String, search_string: &str, _flags: &Vec<String>) -> Vec<String> {
    let mut matched_lines = vec![];

    for line in text_to_search.lines().map(String::from) {
        if line.contains(search_string) {
            matched_lines.push(line);
        }
    }

    println!("{:?}", matched_lines);

    matched_lines
}
