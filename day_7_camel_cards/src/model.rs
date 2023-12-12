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

pub enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl Card {
    fn get_value(&self) -> i8 {
        match self {
            Card::Two => 2,
            Card::Three => 3,
            Card::Four => 4,
            Card::Five => 5,
            Card::Six => 6,
            Card::Seven => 7,
            Card::Eight => 8,
            Card::Nine => 9,
            Card::Ten => 10,
            Card::Jack => 11,
            Card::Queen => 12,
            Card::King => 13,
            Card::Ace => 14,
        }
    }

    fn is_higher_value(&self, other: &Card) -> bool {
        self.get_value() > other.get_value()
    }
}

pub struct Hand {
    pub cards: Vec<Card>,
}


impl Hand {
    
}