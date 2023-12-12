use crate::model::EngineParser;

pub fn run(schematic: &Vec<String>) -> i32 {
    let symbols = EngineParser::get_symbols(schematic, &EngineParser::is_gear);

    let part_numbers = EngineParser::get_part_numbers(schematic, &symbols, &EngineParser::is_gear);

    let mut gears: Vec<Vec<i32>> = Vec::new();

    let mut count = 0;

    for symbol in symbols.iter() {
        gears.push(Vec::new());
        for part in &part_numbers {
            for part_symbol in &part.associated_symbols {
                if &symbol == &part_symbol {
                    gears[count].push(part.value);
                }
            }
        }

        count += 1;
    }

    let gears: Vec<&Vec<i32>> = gears.iter().filter(|g| g.len() == 2).collect();

    let mut ratios: Vec<i32> = Vec::new();

    for gear in gears.iter() {
        ratios.push(gear[0] * gear[1]);
    }

    ratios.iter().sum()
}