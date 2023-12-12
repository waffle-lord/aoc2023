use crate::model::EngineParser;

pub fn run(schematic: &Vec<String>) -> i32 {
    let symbols = EngineParser::get_symbols(schematic, &EngineParser::is_symbol);

    let part_numbers = EngineParser::get_part_numbers(schematic, &symbols, &EngineParser::is_symbol);

    part_numbers.iter().map(|p| p.value).sum()
}