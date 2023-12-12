use crate::model::Game;

pub fn get_power_min_sum(input: &Vec<String>) -> i32 {
    let mut games: Vec<Game> = Vec::new();

    for line in input.iter() {
        games.push(Game::parse(line));
    }

    let total_power_sum = games.iter().map(|g| g.min_power()).sum::<i32>();

    total_power_sum
}
