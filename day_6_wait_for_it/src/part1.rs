use crate::model::Race;


pub fn run(input: &Vec<String>) -> i64 {

    let races = Race::get_races(input);

    let mut counts: Vec<i64> = Vec::new();

    for r in races.iter() {
        counts.push(r.get_possible_win_count());
    }

    
    counts.iter().product()
}