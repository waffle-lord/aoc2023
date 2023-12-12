pub struct CubeCounts {
    red: i32,
    green: i32,
    blue: i32,
}

impl CubeCounts {
    pub fn new() -> CubeCounts {
        CubeCounts { red: 0, green: 0, blue: 0 }
    }

    pub fn with(red: i32, green: i32, blue: i32) -> CubeCounts {
        CubeCounts { red, green, blue }
    }
}

pub struct Game {
    pub id: i32,
    pub red_min: i32,
    pub green_min: i32,
    pub blue_min: i32,
    pub hands: Vec<CubeCounts>
}

impl Game {
    fn parse_hand(play: &str) -> Option<CubeCounts> {
        let mut parts: Vec<&str> = Vec::new();
        parts.push(play);

        if play.contains(",") {
            parts = play.split(",").collect();
        }

        let mut hand = CubeCounts::new();

        for p in parts.iter() {
            let p: Vec<&str> = p.trim().split(" ").collect();

            //println!("PART -> {:?}", p);

            if p.len() != 2 {
                panic!("a part didn't have 2 sections")
            }

            let amount: i32 = p[0].to_string().parse().expect("failed to parse part amount");
            let color = p[1].trim();

            match color {
                "red" => hand.red = amount,
                "green" => hand.green = amount,
                "blue" => hand.blue = amount,
                _ => {}
            }
        }

        return Some(hand);
    }

    pub fn min_power(&self) -> i32 {
        let power = self.red_min * self.green_min * self.blue_min;

        //println!("Game {} power: {}", self.id, power);

        power
    }

    pub fn validate(&self, bag: &CubeCounts) -> bool {
        for hand in &self.hands {
            if hand.red > bag.red || hand.green > bag.green || hand.blue > bag.blue {
                return false;
            }
        }

        true
    }

    pub fn parse(line: &str) -> Game {
        let data: Vec<&str> = line.split(":").collect();

        if data.len() != 2 {
            panic!("something is wrong with game data");
        }

        //println!("LINE -> {:?}", line);

        let id = data[0].strip_prefix("Game ").expect("failed to parse game id");

        let id = id.parse().expect("failed to convert game id to i32");

        let mut plays: Vec<&str> = Vec::new();

        if data[1].contains(";") {
            plays = data[1].split(";").collect();
        }
        else {
            plays.push(data[1])
        }

        //println!("PLAY => {:?}", plays);

        let mut hands: Vec<CubeCounts> = Vec::new();
        let mut red_min: i32 = 0;
        let mut green_min: i32 = 0;
        let mut blue_min: i32 = 0;

        for play in plays.iter() {
            if let Some(hand) = Game::parse_hand(play) {

                //println!("HAND -> r{} - g{} - b{}", hand.red, hand.green, hand.blue);

                if red_min < hand.red {
                    red_min = hand.red;
                }

                if green_min < hand.green {
                    green_min = hand.green;
                }

                if blue_min < hand.blue {
                    blue_min = hand.blue;
                }

                hands.push(hand);
            };
        }

        Game {
            id,
            red_min,
            green_min,
            blue_min,
            hands,
        }
    }
}