use crate::model;


pub fn run(input: &Vec<String>) -> i64 {
    let directions: &Vec<char> = &input[0].chars().collect();
 
    print!("loading map ... ");
    let map = model::get_map(input);
    println!("done");

    let last_locations: Vec<String> = map.keys().filter(|k| k.ends_with("A")).cloned().collect();

    // GCF is 307

    println!("{:#?}", last_locations);

    let mut location_counts: Vec<i64> = Vec::new();

    for l in last_locations.iter() {
        let mut tl = l.to_string();
        let mut index = 0;
        let mut count = 0;

        loop {
            let d = directions[index];

            index += 1;

            if index >= directions.len() {
                index = 0;
            }

            if d == 'L' {
                tl = map.get(&tl).unwrap().0.to_string();
            }

            if d == 'R' {
                tl = map.get(&tl).unwrap().1.to_string();
            }

            count += 1;

            if tl.ends_with("Z") {
                println!("{} -> {} in {} steps", l, tl, count);
                location_counts.push(count);
                break;
            }
        }
    }

    fn lcm(a: i64, b: i64) -> i64 {
        a / gcd(a, b) * b
    }

    fn gcd(a: i64, b: i64) -> i64 {
        if b == 0 {
            a
        } else {
            gcd(b, a % b)
        }
    }

    let mut a = location_counts.get(0).unwrap().clone();
    
    for i in 1..location_counts.len() {
        a = lcm(a, location_counts.get(i).unwrap().clone());
    }

    a
}
