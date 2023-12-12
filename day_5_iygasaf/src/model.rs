pub struct MapInfo {
    pub dest_start: i64,
    pub source_start: i64,
    pub length: i64,
}

pub struct SeedRange {
    pub start: i64,
    pub length: i64,
}

pub struct Almanac {
    pub seeds: Vec<i64>,
    seed_ranges: Vec<SeedRange>,
    pub seed_to_soil: Vec<MapInfo>,
    soil_to_fertilizer: Vec<MapInfo>,
    fertilizer_to_water: Vec<MapInfo>,
    water_to_light: Vec<MapInfo>,
    light_to_temp: Vec<MapInfo>,
    temp_to_humidity: Vec<MapInfo>,
    pub humidity_to_location: Vec<MapInfo>,
}

impl Almanac {
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

    fn is_space(c: char) -> bool {
        c == ' '
    }

    fn is_letter(c: char) -> bool {
        !Almanac::is_digit(c) && !Almanac::is_space(c)
    }

    fn get_numbers(line: &str) -> Vec<i64> {
        let mut buf: Vec<char> = Vec::new();
        let mut line_numbers: Vec<i64> = Vec::new();

        for c in line.chars() {
            if Almanac::is_space(c) {
                if buf.len() == 0 {
                    continue;
                }

                let num: i64 = String::from_iter(buf.iter()).parse().unwrap();
                line_numbers.push(num);
                buf.clear();
                continue;
            }

            if Almanac::is_digit(c) {
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

    fn get_seeds(line: &str) -> Vec<i64> {
        let line = line.strip_prefix("seeds: ").unwrap();

        Almanac::get_numbers(line)
    }

    fn get_seed_ranges(line: &str) -> Vec<SeedRange> {
        let line = line.strip_prefix("seeds: ").unwrap();

        let nums = Almanac::get_numbers(line);

        let mut seed_ranges: Vec<SeedRange> = Vec::new();

        let mut seed_start: i64 = 0;
        let mut pair = false;

        for n in nums {
            if pair {
                pair = false;
                seed_ranges.push(SeedRange {
                    start: seed_start,
                    length: n,
                });

                continue;
            }

            pair = true;
            seed_start = n;
        }

        seed_ranges
    }

    fn load_map_info(map: &mut Vec<MapInfo>, line: &str) {
        let line_numbers: Vec<i64> = Almanac::get_numbers(line);

        if line_numbers.len() != 3 {
            panic!("line should have contained 3 values: {}", line);
        }

        let source_start = line_numbers[1];
        let dest_start = line_numbers[0];
        let length = line_numbers[2];

        map.push(MapInfo {
            dest_start,
            source_start,
            length,
        });
    }

    fn get_mapped_value(map_ranges: &Vec<MapInfo>, input: &i64) -> i64 {
        for map_info in map_ranges.iter() {
            let upper_source_bound = map_info.source_start + map_info.length - 1;
            let upper_dest_bound = map_info.dest_start + map_info.length - 1;

            if input >= &map_info.source_start && input <= &upper_source_bound {
                let offset = upper_source_bound - input;
                let value = upper_dest_bound - offset;

                return value;
            }
        }

        *input
    }

    fn get_mapped_value_invt(map_ranges: &Vec<MapInfo>, input: &i64) -> i64 {
        for map_info in map_ranges.iter() {
            let upper_source_bound = map_info.source_start + map_info.length - 1;
            let upper_dest_bound = map_info.dest_start + map_info.length - 1;

            if input >= &map_info.dest_start && input <= &upper_dest_bound {
                let offset = upper_dest_bound - input;
                let value = upper_source_bound - offset;

                return value;
            }
        }

        *input
    }

    pub fn crawl_maps(&self, seed: &i64) -> i64 {
        let value = Almanac::get_mapped_value(&self.seed_to_soil, seed);
        let value = Almanac::get_mapped_value(&self.soil_to_fertilizer, &value);
        let value = Almanac::get_mapped_value(&self.fertilizer_to_water, &value);
        let value = Almanac::get_mapped_value(&self.water_to_light, &value);
        let value = Almanac::get_mapped_value(&self.light_to_temp, &value);
        let value = Almanac::get_mapped_value(&self.temp_to_humidity, &value);
        Almanac::get_mapped_value(&self.humidity_to_location, &value)
    }

    pub fn crawl_maps_invt(&self, loc: &i64) -> i64 {

        let value = Almanac::get_mapped_value_invt(&self.humidity_to_location, loc);
        let value = Almanac::get_mapped_value_invt(&self.temp_to_humidity, &value);
        let value = Almanac::get_mapped_value_invt(&self.light_to_temp, &value);
        let value = Almanac::get_mapped_value_invt(&self.water_to_light, &value);
        let value = Almanac::get_mapped_value_invt(&self.fertilizer_to_water, &value);
        let value = Almanac::get_mapped_value_invt(&self.soil_to_fertilizer, &value);
        Almanac::get_mapped_value_invt(&self.seed_to_soil, &value)
    }

    pub fn check_seed_ranges(&self, seed: &i64) -> bool {
        for range in self.seed_ranges.iter() {
            if seed >= &range.start && seed < &(range.start + range.length) {
                return true;
            }
        }

        false
    }

    pub fn parse(data: &Vec<String>, use_ranges: bool) -> Almanac {
        let mut almanac = Almanac {
            seeds: Vec::new(),
            seed_ranges: Vec::new(),
            seed_to_soil: Vec::new(),
            soil_to_fertilizer: Vec::new(),
            fertilizer_to_water: Vec::new(),
            water_to_light: Vec::new(),
            light_to_temp: Vec::new(),
            temp_to_humidity: Vec::new(),
            humidity_to_location: Vec::new(),
        };

        let mut current_map: &mut Vec<MapInfo> = &mut almanac.seed_to_soil;

        for line in data.iter() {
            // get seeds
            if line.starts_with("seeds: ") {
                if use_ranges {
                    almanac.seed_ranges = Almanac::get_seed_ranges(line);
                } else {
                    almanac.seeds = Almanac::get_seeds(line);
                    continue;
                }
            }

            // handle blank lines
            if line.is_empty() {
                continue;
            }

            // set currect map when a new map line is found
            match line.as_str() {
                "seed-to-soil map:" => current_map = &mut almanac.seed_to_soil,
                "soil-to-fertilizer map:" => current_map = &mut almanac.soil_to_fertilizer,
                "fertilizer-to-water map:" => current_map = &mut almanac.fertilizer_to_water,
                "water-to-light map:" => current_map = &mut almanac.water_to_light,
                "light-to-temperature map:" => current_map = &mut almanac.light_to_temp,
                "temperature-to-humidity map:" => current_map = &mut almanac.temp_to_humidity,
                "humidity-to-location map:" => current_map = &mut almanac.humidity_to_location,
                _ => {}
            }

            if line.starts_with(&Almanac::is_letter) {
                continue;
            }

            // parse map data into current map
            Almanac::load_map_info(&mut current_map, line);
        }

        almanac
    }
}
