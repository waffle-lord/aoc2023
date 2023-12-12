use crate::model::Almanac;


pub fn run(input: &Vec<String>) -> i64 {
    let mut almanac = Almanac::parse(input, true);

    almanac.humidity_to_location.sort_by(|a, b| b.dest_start.cmp(&a.dest_start));

    let biggest_location = almanac.humidity_to_location[0].dest_start + almanac.humidity_to_location[0].length;

    let mut count = 1000000;

    for i in 0..biggest_location {
        let seed = almanac.crawl_maps_invt(&i);

        if count == 0 {
            count = 1000000;
            println!("loc: {} - r{}", i, biggest_location - i);
        }

        count -= 1;

        if almanac.check_seed_ranges(&seed) {
            println!("found: {}", i);
            return i;
        }
    }

    0
}