use std::{fs, path::PathBuf};
mod part1;
mod part2;
mod model;

const TEST_INPUT: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

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

    let part1_answer = part1::run(&test_input);
    let part2_answer = part2::run();

    println!("==============");
    println!("Part 1 Answer: {}", part1_answer);
    println!("Part 2 Answer: {}", part2_answer);
}
