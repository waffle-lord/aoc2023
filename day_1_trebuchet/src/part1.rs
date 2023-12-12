fn get_first_digit_as_string(line: &str) -> String {
    let mut digit = -1;

    for character in line.chars() {
        digit = match character.to_string().parse() {
            Ok(num) => num,
            Err(_) => -1,
        };

        if digit != -1 {
            break;
        }
    }

    digit.to_string()
}

fn get_last_digit_as_string(line: &str) -> String {
    let reversed_line = line.chars().rev().collect::<String>();
    get_first_digit_as_string(&reversed_line)
}

pub fn parse_number(line: &str) -> Result<i32, &str> {
    println!("Parsing '{}'", line);

    let mut first = get_first_digit_as_string(line);
    let last = get_last_digit_as_string(line);

    println!("first: {}  -  last: {}", first, last);

    first.push_str(&last);

    let number = first;

    match number.trim().parse() {
        Ok(num) => Ok(num),
        Err(_) => Err("failed to parse number"),
    }
}
