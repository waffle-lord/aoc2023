use crate::model;


pub fn run(input: &Vec<String>) -> i64 {
    let directions: &Vec<char> = &input[0].chars().collect();
 
    print!("loading map ... ");
    let map = model::get_map(input);
    println!("done");

    let mut count = 1000000;
    let mut step_count = 0;
    let mut last_locations: Vec<String> = map.keys().filter(|k| k.ends_with("A")).cloned().collect();

    println!("Start Keys ::\n{:#?}", last_locations);

    // probably have to do math or something
    // - greatest common divisor? idk ...

    step_count
}
