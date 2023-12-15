use crate::model;

pub fn run(input: &Vec<String>) -> i64 {
    let hands = model::get_ordered_hands(input, true);

    let mut total: i64 = 0;

    for i in 0..hands.len() {
        let multiplier: i64 = (i + 1) as i64;

        println!("r:{} * b:{} = {} :: {:?} -> {}", multiplier, hands[i].bid, hands[i].bid * multiplier, hands[i].strength, hands[i].original);

        total += hands[i].bid * multiplier;
    }

    total
}
