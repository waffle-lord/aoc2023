use crate::model::Almanac;


pub fn run(input: &Vec<String>) -> i64 {
    let almanac = Almanac::parse(input, false);

    let mut locations: Vec<i64> = Vec::new();

    for seed in almanac.seeds.iter() {
        locations.push(almanac.crawl_maps(seed));
    }

    *locations.iter().min().unwrap()
}