#[derive(Debug)]
pub struct Point {
    x: i32,
    y: i32,
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

#[derive(Debug)]
pub struct Number {
    pub is_next_to_symbol: bool,
    pub value: i32,
    pub start: i32,
    pub end: i32,
    pub y: i32,
    pub associated_symbols: Vec<Point>,
}

pub struct EngineParser {}

impl EngineParser {
    fn is_digit(c: char) -> bool {
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

    fn is_dot(c: char) -> bool {
        c == '.'
    }

    pub fn is_gear(c: char) -> bool {
        c == '*'
    }

    pub fn is_symbol(c: char) -> bool {
        !EngineParser::is_dot(c) && !EngineParser::is_digit(c)
    }

    fn check_symbols_above_or_below(symbols: &Vec<Point>, num: &mut Number) -> bool {
        let top_left = Point {
            x: num.start - 1,
            y: num.y + 1,
        };
        let top_right = Point {
            x: num.end + 1,
            y: num.y + 1,
        };
        let bottom_left = Point {
            x: num.start - 1,
            y: num.y - 1,
        };
        let bottom_right = Point {
            x: num.end + 1,
            y: num.y - 1,
        };

        let mut symbols_found = false;


        if symbols.contains(&top_left) {
            num.associated_symbols.push(top_left);
            symbols_found = true;
        }

        if symbols.contains(&top_right) {
            num.associated_symbols.push(top_right);
            symbols_found = true;
        }

        if symbols.contains(&bottom_left) {
            num.associated_symbols.push(bottom_left);
            symbols_found = true;
        }

        if symbols.contains(&bottom_right) {
            num.associated_symbols.push(bottom_right);
            symbols_found = true;
        }


        for i in num.start..num.end + 1 {
            let point_above = Point { x: i, y: num.y + 1 };
            let point_below = Point { x: i, y: num.y - 1 };

            if symbols.contains(&point_above) {
                num.associated_symbols.push(point_above);
                symbols_found = true;
            }

            if symbols.contains(&point_below) {
                num.associated_symbols.push(point_below);
                symbols_found = true;
            }
        }

        symbols_found
    }

    fn get_number(line: &String, x_start: usize, y: i32, previous_was_symbol: bool, f: &dyn Fn(char) -> bool) -> Number {

        let mut x_start = x_start;
        let start = x_start;
        let mut end = 0;
        let mut next_to_symbol = previous_was_symbol;
        let mut value: Vec<char> = Vec::new();
        let len = line.chars().count();

        let line: Vec<char> = line.chars().collect();

        let mut associated_symbols : Vec<Point> = Vec::new();

        if previous_was_symbol {
            associated_symbols.push(Point {x: (start - 1).try_into().unwrap(), y: y});
        }

        value.push(line[x_start]);

        while x_start < len {
            if x_start + 1 == len {
                end = x_start;
                break;
            }

            x_start += 1;

            if EngineParser::is_digit(line[x_start]) {
                value.push(line[x_start])
            } else if f(line[x_start]) {
                next_to_symbol = true;
                associated_symbols.push(Point {x: x_start.try_into().unwrap(), y: y});
                end = x_start - 1;
                break;
            } else {
                end = x_start - 1;
                break;
            }
        }

        let value: String = value.iter().collect();

        let value: i32 = value.parse().unwrap();

        Number {
            value: value,
            start: start.try_into().unwrap(),
            end: end.try_into().unwrap(),
            is_next_to_symbol: next_to_symbol,
            y,
            associated_symbols,
        }
    }

    pub fn get_symbols(schematic: &Vec<String>, f: &dyn Fn(char) -> bool) -> Vec<Point> {

        let mut symbols: Vec<Point> = Vec::new();

        for (y, line) in schematic.iter().enumerate() {
            for (x, c) in line.chars().enumerate() {
                if f(c) {
                    symbols.push(Point {
                        x: x.try_into().unwrap(),
                        y: y.try_into().unwrap(),
                    });
                }
            }
        }

        symbols
    }

    pub fn get_part_numbers(schematic: &Vec<String>, symbols: &Vec<Point>, symbol_check: &dyn Fn(char) -> bool) -> Vec<Number> {
        let mut part_numbers: Vec<Number> = Vec::new();

        for (y, line) in schematic.iter().enumerate() {
            let len = line.chars().count();

            let mut line_done = false;
            let mut line_pos = 0;
            let mut previous_was_symbol = false;

            while !line_done {
                for (x, c) in line[line_pos..len].chars().enumerate() {

                    if x + line_pos >= len - 1 {
                        line_done = true;
                    }

                    if EngineParser::is_dot(c) {
                        previous_was_symbol = false;
                        continue;
                    }

                    if symbol_check(c) {
                        previous_was_symbol = true;
                        continue;
                    }

                    if EngineParser::is_digit(c) {
                        let mut num = EngineParser::get_number(
                            &line,
                            x + line_pos,
                            y.try_into().unwrap(),
                            previous_was_symbol,
                            &symbol_check,
                        );

                        let num_end = num.end;

                        if num.is_next_to_symbol {
                            part_numbers.push(num);
                        } else if EngineParser::check_symbols_above_or_below(&symbols, &mut num) {
                            part_numbers.push(num);
                        }

                        line_pos = (num_end + 1).try_into().unwrap();

                        previous_was_symbol = false;

                        line_done = line_pos >= len - 1;
                        
                        break;
                    }
                }
            }
        }

        part_numbers
    }
}
