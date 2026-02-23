mod grid;
mod parser;

use crate::{
    grid::{FactoryMap, Matrix},
    parser::InputParser,
};

fn main() {
    let grid_data: Matrix = InputParser::read_data("day4/data/input.txt");
    let mut factory_map = FactoryMap::new(grid_data);

    let all_accessible = factory_map.count_all_accessible_papers();

    println!("Accesible positions: {}", all_accessible.unwrap().len());

    let mut total_accessed = 0;
    while let Some(accessible_papers) = factory_map.count_all_accessible_papers() {
        total_accessed += accessible_papers.len();
        factory_map.update_grid(accessible_papers);
    }

    println!("Total accessed positions: {}", total_accessed);
}
