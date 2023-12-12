use crate::model::Card;

pub fn run(input: &Vec<String>) -> i32 {
    let mut cards: Vec<Card> = Vec::new();

    for line in input.iter() {
        cards.push(Card::parse(line));
    }

    for i in 0..cards.len() {
        for _ in 0..cards[i].count {   
            for n in 0..cards[i].matching_count {
                let next = n as usize;
                
                let next = i + next + 1;

                if next > cards.len() {
                    break;
                }
                
                cards[next].count += 1;
            }
        }
    }

    cards.iter().map(|c| c.count).sum()
}