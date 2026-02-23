mod grid;
mod parser;

use crate::{
    grid::{FactoryMap, Matrix},
    parser::InputParser,
};

fn main() {
    let grid_data: Matrix = InputParser::read_data("day4/data/input.txt");
    let factory_map = FactoryMap::new(grid_data);

    let all_accessible = factory_map.count_all_accessible_positions();

    println!("Accesible positions: {}", all_accessible);
}
