use crate::model;


pub fn run(input: &Vec<String>) -> i64 {
    let directions: &Vec<char> = &input[0].chars().collect();
 
    print!("loading map ... ");
    let map = model::get_map(input);
    println!("done");

    let mut step_count = 0;
    let mut last_location = "AAA";

    let mut direction_index = 0;

    loop {
        let d = directions[direction_index];

        direction_index += 1;

        if direction_index == directions.len() {
            direction_index = 0;
        }

        step_count += 1;

        if d == 'L' {
            print!("steps:{} :: L | '{}' -> ", step_count, last_location);

            last_location = &map.get(last_location).unwrap().0;

            println!("'{}'", last_location)
        }
        else if d == 'R' {
            print!("steps:{} :: R | '{}' -> ", step_count, last_location);

            last_location = &map.get(last_location).unwrap().1;

            println!("'{}'", last_location)
        }

        if last_location == "ZZZ" {
            break;
        }
    }

    step_count
}
