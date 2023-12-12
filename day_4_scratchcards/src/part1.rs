use crate::model;


pub fn run(input: &Vec<String>) -> i32 {
    let mut scores: Vec<i32> = Vec::new();

    for line in input.iter() {
        let card = model::Card::parse(line);

        scores.push(card.score);
    }

    scores.iter().sum()
}