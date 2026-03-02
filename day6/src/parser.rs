use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use crate::calculator::OperationType;

pub struct InputParser {}

impl InputParser {
    pub fn read_data(filename: &str) -> (Vec<Vec<u64>>, Vec<OperationType>) {
        let file = File::open(filename).expect("Provided file does not exist");
        let reader = BufReader::new(file);

        let mut operands: Vec<Vec<u64>> = vec![];
        let mut operations: Vec<OperationType> = vec![];

        let data = reader.lines();

        for line in data.map_while(Result::ok) {
            if line.contains('+') || line.contains('*') {
                operations = line
                    .split_whitespace()
                    .map(|e| OperationType::from(e))
                    .collect();
            } else {
                operands.push(
                    line.split_whitespace()
                        .map(|value| value.parse::<u64>().unwrap())
                        .collect::<Vec<u64>>(),
                );
            }
        }

        (operands, operations)
    }
}
