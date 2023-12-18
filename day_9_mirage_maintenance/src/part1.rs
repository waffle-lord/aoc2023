use crate::model::{Lexer, get_next_number};


pub fn run(input: &Vec<String>) -> i64 {

    let mut sequences: Vec<Vec<i64>> = Vec::new();
    let mut next_values: Vec<i64> = Vec::new();

    for line in input.iter() {
        sequences.push(Lexer::get_numbers(line));
    }


    for s in sequences.iter() {
        let next = get_next_number(s);

        next_values.push(next);
    }


    next_values.iter().sum()
}
