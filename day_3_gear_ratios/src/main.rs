use std::{fs, path::PathBuf};
mod model;
mod part1;
mod part2;

const TEST_INPUT_2: &str = "467..114......
...*.......993
..35..633..*..
......#....*..
617*..........
.....+.58*32..
..592........9
......755...*.
...$.*........
*664.598......";

const TEST_INPUT: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

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
    //let input = TEST_INPUT_2.lines().map(String::from).collect();
    let input = read_lines(INPUT_FILE);

    let part_1_answer = part1::run(&input);
    let part_2_answer = part2::run(&input);

    println!("Part 1 answer: {}", part_1_answer);
    println!("Part 2 answer: {}", part_2_answer);
}
