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

    pub fn get_single_number(line: &str) -> i64 {
        let mut buf: Vec<char> = Vec::new();

        for c in line.chars() {
            if Lexer::is_digit(c) {
                buf.push(c);
            }
        }

        let num: i64 = String::from_iter(buf.iter()).parse().unwrap();

        num
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

pub struct Race {
    time: i64,
    distance: i64,
}

impl Race {
    pub fn get_possible_win_count(&self) -> i64 {

        let mut time = self.time;

        let mut possible_count = 0;

        for n in 0..self.time {
            let result = n * time;

            if result > self.distance {
                possible_count += 1;
            }

            time -= 1;
        }

        possible_count
    }

    pub fn get_single_race(input: &Vec<String>) -> Race {
        let mut time: i64 = 0;
        let mut distance: i64 = 0;

        for line in input.iter() {

            if line.starts_with("Time:") {
                let line = line.strip_prefix("Time:").unwrap();

                time = Lexer::get_single_number(line);
            }

            if line.starts_with("Distance:") {
                let line = line.strip_prefix("Distance:").unwrap();

                distance = Lexer::get_single_number(line);
            }
        }

        Race { time, distance }
    }

    pub fn get_races(input: &Vec<String>) -> Vec<Race> {

        let mut times: Vec<i64> = Vec::new();
        let mut distances: Vec<i64> = Vec::new();
        let mut races: Vec<Race> = Vec::new();

        for line in input.iter() {

            if line.starts_with("Time:") {
                let line = line.strip_prefix("Time:").unwrap();

                times = Lexer::get_numbers(line);
            }

            if line.starts_with("Distance:") {
                let line = line.strip_prefix("Distance:").unwrap();

                distances = Lexer::get_numbers(line);
            }
        }

        if times.len() != distances.len() {
            panic!("times and distances should be equal length");
        }

        for i in 0..times.len() {
            races.push(Race { time: times[i], distance: distances[i] });
        }

        races
    }
}