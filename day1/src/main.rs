mod parser;
mod resolver;
use parser::DataParser;

use crate::resolver::Solver;

fn main() {
    let data = DataParser::parse_data("day1/data/input.txt");

    let mut solver = Solver::new(50, data);
    let result = solver.solve();

    println!("Result D1.1: {}", result);
}
