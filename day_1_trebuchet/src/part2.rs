use std::collections::HashMap;

fn update_data(
    current_first: &mut (i32, String),
    current_last: &mut (i32, String),
    next: (usize, &str),
) {
    match i32::try_from(next.0) {
        Ok(num) => {
            if current_first.0 == -1 || num < current_first.0 {
                current_first.0 = num;
                current_first.1 = next.1.to_string();
                println!("udpated first: {:?}", current_first);
            }

            if current_last.0 == -1 || num > current_last.0 {
                current_last.0 = num;
                current_last.1 = next.1.to_string();
                println!("updated last: {:?}", current_last);
            }
        }
        Err(_) => println!("something went wrong"),
    }
}

fn get_single_match<'a>(line: &'a str, input: &'a str, last: bool) -> Option<(usize, &'a str)> {
    //println!("Input: '{}'", input);
    let v: Vec<_> = line.match_indices(input).collect();

    let data;

    if last {
        data = match v.last() {
            Some(first) => Some(first),
            None => None,
        };
    } else {
        data = match v.first() {
            Some(first) => Some(first),
            None => None,
        };
    }

    data.copied()
}

fn get_digits(line: &str, map: &HashMap<&str, i32>) -> (String, String) {
    let mut first: (i32, String) = (-1, String::from(""));
    let mut last: (i32, String) = (-1, String::from(""));

    for item in map.iter() {
        if let Some(next) = get_single_match(line, item.0, false) {
            update_data(&mut first, &mut last, (next.0, &item.1.to_string()[..]));
        };

        if let Some(next) = get_single_match(line, item.0, true) {
            update_data(&mut first, &mut last, (next.0, &item.1.to_string()[..]));
        };

        if let Some(next) = get_single_match(line, &item.1.to_string(), false) {
            update_data(&mut first, &mut last, (next.0, &item.1.to_string()[..]));
        };

        if let Some(next) = get_single_match(line, &item.1.to_string(), true) {
            update_data(&mut first, &mut last, (next.0, &item.1.to_string()[..]));
        };
    }

    (first.1, last.1)
}

pub fn parse_number(line: &str) -> Result<i32, &str> {
    let digit_map: HashMap<&str, i32> = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);

    println!("Parsing '{}'", line);

    let digits = get_digits(line, &digit_map);

    println!("first: {}  -  last: {}", digits.0, digits.1);

    let number = format!("{}{}", digits.0, digits.1);

    match number.trim().parse() {
        Ok(num) => Ok(num),
        Err(_) => Err("failed to parse number"),
    }
}
