use std::{path::PathBuf, fs};
mod model;
mod part1;
mod part2;

// const TEST_INPUT_PART1: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
// Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
// Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
// Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
// Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";


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
    //let input: Vec<String> = TEST_INPUT_PART1.lines().map(String::from).collect();
    let input: Vec<String> = read_lines(INPUT_FILE);

    let bag = model::CubeCounts::with(12, 13, 14);

    let part_1_answer = part1::get_possible(bag, &input);
    let part_2_answer = part2::get_power_min_sum(&input);

    println!("=================");
    println!("Part 1 answer: {}", part_1_answer);
    println!("Part 2 answer: {}", part_2_answer);
}
