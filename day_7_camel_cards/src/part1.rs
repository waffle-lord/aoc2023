use crate::model::Hand;


pub fn run(input: &Vec<String>) -> i64 { 
    let mut hands: Vec<Hand> = Vec::new();


    for line in input.iter() {
        hands.push(Hand::parse(line));
    }

    println!("{:#?}", {hands});

    todo!()
}
