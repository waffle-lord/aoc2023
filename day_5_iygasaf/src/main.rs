use std::{fs, path::PathBuf};
mod part1;
mod part2;

const TEST_INPUT: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

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
