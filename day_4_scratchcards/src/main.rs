use std::{fs, path::PathBuf};
mod model;
mod part1;
mod part2;

const TEST_INPUT: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

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

    let part1_answer = part1::run(&input);
    let part2_answer = part2::run(&input);

    println!("==============");
    println!("Part 1 Answer: {}", part1_answer);
    println!("Part 2 Answer: {}", part2_answer);
}
