
pub struct Card {
    pub id: i32,
    pub score: i32,
    pub matching_count: i32,
    pub winning_numbers: Vec<i32>,
    pub card_numbers: Vec<i32>,
    pub count: i32,
}

impl Card {

    fn is_space(c: char) -> bool {
        c == ' '
    }   

    fn is_digit(c: char) -> bool {
        c == '0' ||
        c == '1' ||
        c == '2' ||
        c == '3' ||
        c == '4' ||
        c == '5' ||
        c == '6' ||
        c == '7' ||
        c == '8' ||
        c == '9'
    }

    fn flush_buf(buf: &mut Vec<char>, output_vec: &mut Vec<i32>) {
        let num: i32 = String::from_iter(buf.iter()).parse().unwrap();
        //println!("--> {}", num);
        output_vec.push(num);
        buf.clear();
    }

    fn parse_numbers(input: &str) -> Vec<i32> {
        let mut buf: Vec<char> = Vec::new();
        let mut output: Vec<i32> = Vec::new();

        for c in input.chars() {

            if Card::is_space(c) {
                if buf.len() > 0 {
                    Card::flush_buf(&mut buf, &mut output)
                }

                continue;
            }

            if Card::is_digit(c) {
                buf.push(c);
            }
        }

        if buf.len() > 0 {
            Card::flush_buf(&mut buf, &mut output)
        }

        output
    }

    fn calc_score(&mut self) {
        for n in self.card_numbers.iter() {
            if self.winning_numbers.contains(n) {
                if self.score == 0 {
                    self.score = 1
                }
                else {
                    self.score = self.score * 2;
                }

                self.matching_count += 1;
            }
        }
    }

    pub fn parse(line: &str) -> Card {

        let card: Vec<String> = line.split(": ").map(String::from).collect();

        let id = card[0].strip_prefix("Card").unwrap();
        let numbers: Vec<String> = card[1].split(" | ").map(String::from).collect();

        let id: i32 = id.trim().parse().unwrap();

        let winning_numbers = Card::parse_numbers(&numbers[0]);
        let card_numbers = Card::parse_numbers(&numbers[1]);

        let mut card = Card {
            id,
            score: 0,
            matching_count: 0,
            winning_numbers,
            card_numbers,
            count: 1
        };

        card.calc_score();

        card
    }
}