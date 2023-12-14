use crate::model::Hand;


pub fn run(input: &Vec<String>) -> i64 { 
    let mut hands: Vec<Hand> = Vec::new();


    for line in input.iter() {
        hands.push(Hand::parse(line));
    }

    hands.sort_by(|a, b| a.strength.get_value().cmp(&b.strength.get_value()));

    // this is all fucked, I have no idea how to swap values in an array in rust :(
    for i in 0..hands.len() {
        let mut current_hand = hands[i];

        current_hand.rank = i.try_into().unwrap();

        if i == 0 {
            continue;
        }

        let mut previous_hand = hands[i - 1];

        if previous_hand.strength.get_value() == current_hand.strength.get_value() {
            for n in 0..current_hand.cards.len() {
                if previous_hand.cards[n].is_higher_value(&current_hand.cards[n]) {
                    hands[i] = previous_hand;
                    hands[i-1] = current_hand;
                }
            }
        }
    }

    
    for hand in hands.iter() {
        println!("rank: {} - {:?}", hand.rank, hand.strength);
    }
    todo!()
}
