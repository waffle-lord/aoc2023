use std::{fs, path::PathBuf};
mod part1;
mod part2;

// const TEST_INPUT_PART1: &str = "1abc2
// pqr3stu8vwx
// a1b2c3d4e5f
// treb7uchet";

// const TEST_INPUT_PART2: &str = "two1nine
// eightwothree
// abcone2threexyz
// xtwone3four
// 4nineeightseven2
// zoneight234
// 7pqrstsixteen";

const INPUT_FILE: &str = "./src/input.txt";

fn read_lines(filename: &str) -> Vec<String> {
    let path = PathBuf::from(filename);

    let file_path = fs::canonicalize(path).unwrap();

    println!("File: {:?}", file_path);

    fs::read_to_string(file_path) 
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

fn run_part_1(input: &Vec<String>) -> String {
    let mut numbers: Vec<i32> = Vec::new();

    for line in input {
        match part1::parse_number(&line) {
            Ok(num) => {
                println!("Number: {}", num);
                numbers.push(num);
            }
            Err(_) => println!("could not find any digits in string")
        };

        println!("------");
    }

    let total: i32= numbers.iter().sum();

    println!("The total is {}", total);

    total.to_string()
}

fn run_part_2(input: &Vec<String>) -> String {
    let mut numbers: Vec<i32> = Vec::new();

    for line in input {
        match part2::parse_number(&line) {
            Ok(num) => {
                println!("Number: {}", num);
                numbers.push(num);
            }
            Err(_) => println!("could not find any digits in string")
        };

        println!("------");
    }

    let total: i32= numbers.iter().sum();

    println!("The total is {}", total);

    total.to_string()
}

fn main() {
    let input: Vec<String> = read_lines(INPUT_FILE);

    let part1_answer = run_part_1(&input);
    let part2_answer = run_part_2(&input);

    println!("==============");
    println!("Part 1 Answer: {}", part1_answer);
    println!("Part 2 Answer: {}", part2_answer);
}
