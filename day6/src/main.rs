mod calculator;
mod parser;

use parser::InputParser;

use crate::calculator::Calculator;

fn main() {
    let (operands, operations) = InputParser::read_data("day6/data/input.txt");

    let calculator = Calculator::new(operands, operations);

    let total = calculator.resolve();
    println!("Total: {}", total);

    let (cephalopod_operands, cephalopod_operations) =
        InputParser::read_cephalopod_data("day6/data/input.txt");

    let cephalopod_total =
        Calculator::resolve_cephalopod(cephalopod_operands, cephalopod_operations);
    println!("Total cephalopod: {}", cephalopod_total);
}
