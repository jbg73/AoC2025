mod batteries_selector;
mod parser;
use batteries_selector::BatterySelector;
use parser::data_parser::DataParser;

fn main() {
    let data = DataParser::parse_data("day3/data/input.txt");

    let mut result = 0;
    for battery_bank in data {
        result = result + BatterySelector::max_joltage_selector(&battery_bank, 12);
    }
    println!("{}", result);
}
