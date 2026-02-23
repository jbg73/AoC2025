use crate::grid::{Cell, CellState, Matrix, Position, Row};
use std::{fs::File, io::BufRead, io::BufReader};
pub struct InputParser {}

impl InputParser {
    pub fn read_data(filename: &str) -> Matrix {
        let file = File::open(filename).expect("File could not be opened!");
        let reader = BufReader::new(file);

        let lines = reader.lines(); // todo: without importing BufRead doesn't work

        lines
            .into_iter()
            .enumerate()
            .map(|row| {
                row.1
                    .unwrap()
                    .chars()
                    .enumerate()
                    .map(|c| Cell::new(Position(row.0, c.0), CellState::from(&c.1)))
                    .collect::<Row>()
            })
            .collect()
    }
}
