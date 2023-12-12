use crate::model::Race;


pub fn run(input: &Vec<String>) -> i64 {

    let race = Race::get_single_race(input);

    race.get_possible_win_count()
}