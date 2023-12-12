use std::{fs, path::PathBuf};
mod part1;
mod part2;
mod model;

const TEST_INPUT: &str = "";

const INPUT_FILE: &str = "./src/input.txt";

fn read_lines(filename: &str) -> Vec<String> {
    let path = PathBuf::from(filename);

    let file_path = fs::canonicalize(path).unwrap();

    fs::read_to_string(file_path) 
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

fn main() {
    let test_input: Vec<String> = TEST_INPUT.lines().map(String::from).collect();
    let input: Vec<String> = read_lines(INPUT_FILE);

    let part1_answer = part1::run();
    let part2_answer = part2::run();

    println!("==============");
    println!("Part 1 Answer: {}", part1_answer);
    println!("Part 2 Answer: {}", part2_answer);
}
