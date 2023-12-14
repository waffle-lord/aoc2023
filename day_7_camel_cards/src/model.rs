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

#[derive(Debug)]
pub enum Strength {
    Unknown,
    HighCard, // 5 counts
    OnePair, // 4 counts
    TwoPair, // 3 counts
    ThreeOfKind, // 3 counts
    FullHouse, // 2 counts
    FourOfKind, // 2 counts
    FiveOfKind, // 1 counts
}

impl Strength {
    pub fn get_value(&self) -> i8 {
        match self {
            Strength::HighCard => 0,
            Strength::OnePair => 1,
            Strength::TwoPair => 2,
            Strength::ThreeOfKind => 3,
            Strength::FullHouse => 4,
            Strength::FourOfKind => 5,
            Strength::FiveOfKind => 6,
            _ => -1
        }
    }
    pub fn from_cards(cards: &Vec<Card>) -> Strength {
        let mut counts: HashMap<i8, i8> = HashMap::new();

        for c in cards.iter() {
            match counts.get_mut(&c.get_value()) {
                Some(v) => *v += 1,
                None => {counts.insert(c.get_value(), 1);},
            }
        }

        match counts.len() {
            5 => Strength::HighCard,
            4 => Strength::OnePair,
            3 => {
                if counts.values().any(|i| *i == 3) {
                    return Strength::ThreeOfKind;
                }
                
                return Strength::TwoPair;
            },
            2 => {
                if counts.values().any(|i| *i == 4) {
                    return Strength::FourOfKind;
                }

                return Strength::FullHouse;
            },
            1 => Strength::FiveOfKind,
            _ => Strength::Unknown,
        }
    }
}

#[derive(Debug)]
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
    pub fn from_char(c: char) -> Option<Card> {
        match c {
         '2' => Some(Card::Two), 
         '3' => Some(Card::Three),
         '4' => Some(Card::Four),
         '5' => Some(Card::Five),
         '6' => Some(Card::Six),
         '7' => Some(Card::Seven),
         '8' => Some(Card::Eight),
         '9' => Some(Card::Nine),
         'T' => Some(Card::Ten),
         'J' => Some(Card::Jack),
         'Q' => Some(Card::Queen),
         'K' => Some(Card::King),
         'A' => Some(Card::Ace),
         _ => None,
        }
    }

    fn get_value(&self) -> i8 {
        match self {
            Card::Two => 0,
            Card::Three => 1,
            Card::Four => 2,
            Card::Five => 3,
            Card::Six => 4,
            Card::Seven => 5,
            Card::Eight => 6,
            Card::Nine => 7,
            Card::Ten => 8,
            Card::Jack => 9,
            Card::Queen => 10,
            Card::King => 11,
            Card::Ace => 12,
        }
    }

    pub fn is_higher_value(&self, other: &Card) -> bool {
        self.get_value() > other.get_value()
    }
}

#[derive(Debug)]
pub struct Hand {
    pub rank: i64,
    pub strength: Strength,
    pub cards: Vec<Card>,
}


impl Hand {
    pub fn parse(line: &String) -> Hand {
        let card_info: Vec<&str> = line.split(" ").collect();
        let cards = card_info[0];
        let bid = card_info[1];

        let mut hand = Hand {
            rank: -1,
            strength: Strength::Unknown,
            cards: Vec::new(),
        };

        for c in cards.chars() {
            let card = Card::from_char(c).unwrap();
            hand.cards.push(card);
        }

        hand.strength = Strength::from_cards(&hand.cards);

        hand
    }
}










