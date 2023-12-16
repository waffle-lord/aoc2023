use std::collections::HashMap;

pub struct Lexer {}

impl Lexer {
	pub fn is_digit(c: char) -> bool {
        c == '0'
            || c == '1'
            || c == '2'
            || c == '3'
            || c == '4'
            || c == '5'
            || c == '6'
            || c == '7'
            || c == '8'
            || c == '9'
    }

    pub fn is_space(c: char) -> bool {
        c == ' '
    }

    pub fn get_numbers(line: &str) -> Vec<i64> {
        let mut buf: Vec<char> = Vec::new();
        let mut line_numbers: Vec<i64> = Vec::new();

        for c in line.chars() {
            if Lexer::is_space(c) {
                if buf.len() == 0 {
                    continue;
                }

                let num: i64 = String::from_iter(buf.iter()).parse().unwrap();
                line_numbers.push(num);
                buf.clear();
                continue;
            }

            if Lexer::is_digit(c) {
                buf.push(c)
            }
        }

        if buf.len() > 0 {
            let num: i64 = String::from_iter(buf.iter()).parse().unwrap();
            line_numbers.push(num);
            buf.clear();
        }

        line_numbers
    }
}

// ----------------------- ----------------------- ----------------------- -----------------------

pub fn get_map(input: &Vec<String>) -> HashMap<String, (String,String)> {

    let mut map: HashMap<String, (String, String)> = HashMap::new();

    for line in input.iter() {

        if line.contains("=") {
            let data: Vec<&str> = line.split("=").collect();
            let location = data[0].trim();
            let mut left_right_options = data[1];
            let left_right_options: Vec<&str> = left_right_options
                .trim()
                .strip_prefix("(").unwrap()
                .strip_suffix(")").unwrap()
                .split(',').collect();

            map.insert(location.to_string(), 
                       (
                           left_right_options[0].trim().to_string(), 
                           left_right_options[1].trim().to_string()
                       ));
        }
    }

    map
}
















