use std::fs::read_to_string;
use regex::Regex;

pub fn read_lines(filename: &str) -> String {
    read_to_string(filename)
        .expect("Expected to read the file")
}

pub fn get_input(filepath: &str, label: &str) -> String {
    let regex = Regex::new(r"src[/\\](aoc\d{4})[/\\](day\d{1,2})").unwrap();
    let captures = regex.captures(filepath).unwrap();
    let aoc = captures.get(1).unwrap().as_str();
    let day = captures.get(2).unwrap().as_str();

    let input_file = format!("../inputs/{aoc}/{day}/aoc-{day}{label}-input.txt");
    let input_content = read_lines(input_file.as_str());
    input_content
}
