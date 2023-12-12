use crate::model::{CubeCounts, Game};

pub fn get_possible(bag: CubeCounts, input: &Vec<String>) -> i32 {
    let mut games: Vec<Game> = Vec::new();

    for line in input.iter() {
        games.push(Game::parse(line));
    }

    let games: Vec<Game> = games
        .into_iter()
        .filter(|g| g.validate(&bag))
        .collect();

        let valid_id_sum = games.iter().map(|g| g.id).sum::<i32>();

        valid_id_sum
}
