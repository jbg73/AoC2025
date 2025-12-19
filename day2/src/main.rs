mod parser;
mod scanner;

use parser::DataParser;
use scanner::Scanner;

use crate::parser::data_parser::Range;

fn main() {
    let data = DataParser::parse_data("day2/data/input.txt");

    let mut invalid_ids_sum = 0;
    for range in data {
        invalid_ids_sum = invalid_ids_sum + Scanner::scan(&range);
    }
    println!("Invalid ids sum: {}", invalid_ids_sum);
}
