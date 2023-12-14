use std::{ops::Index, collections::HashMap};

use crate::model::Hand;

pub fn run(input: &Vec<String>) -> i64 { 
    let mut hands: Vec<Hand> = Vec::new();
    let mut ordered_hands: HashMap<i8, Vec<&Hand>> = HashMap::new();

    // get all hands
    for line in input.iter() {
        hands.push(Hand::parse(line));
    }

    for i in 0..hands.len() {
        match ordered_hands.get_mut(&hands[i].strength.get_value()) {
            Some(v) => v.push(&hands[i]),
            None => {
                let mut new_hands: Vec<&Hand> = Vec::new();
                new_hands.push(&hands[i]);
                ordered_hands.insert(hands[i].strength.get_value(), new_hands);
            },
        }
    }

    for strength_set in ordered_hands.values_mut() {
        // just continue if the strength set only has 1 value. Nothing to order
        if strength_set.len() == 1 {
            continue;
        }

        strength_set.sort_by(|a, b| a.get_value().cmp(&b.get_value()));
    }

    let mut ordered_hands: Vec<(i8, Vec<&Hand>)> = ordered_hands
        .into_iter()
        .collect();

    ordered_hands.sort_by(|a, b| a.0.cmp(&b.0));

    let hands: Vec<&Hand> = ordered_hands
        .into_iter()
        .map(|i| i.1)
        .flatten()
        .collect();

    let mut total: i64 = 0;

    for i in 0..hands.len() {
        let multiplier: i64 = (i + 1) as i64;

        total += hands[i].bid * multiplier;
    }


    total
}
