use std::{
    fs::File,
    io::{BufRead, BufReader},
    thread::current,
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

    pub fn read_cephalopod_data(filename: &str) -> (Vec<Vec<u64>>, Vec<OperationType>) {
        let file = File::open(filename).expect("Provided file does not exist");
        let reader = BufReader::new(file);

        let data = reader.lines();
        let (filled_data, operations) = InputParser::fill_cephalopod_data(data);

        let rows = filled_data.len();
        let cols = filled_data[0].len();

        let mut operands: Vec<Vec<u64>> = vec![];
        let mut single_problem_operands: Vec<u64> = vec![];
        let mut raw_num;
        for j in (0..cols).rev() {
            raw_num = String::new();
            for i in 0..rows {
                let current_char = filled_data[i][j];
                if current_char == 'X' || current_char == '*' || current_char == '+' {
                    continue;
                }
                if current_char.to_string().as_str().parse::<u64>().is_ok() {
                    raw_num.push(current_char);
                }
            }
            if raw_num.is_empty() {
                operands.push(single_problem_operands.clone());
                single_problem_operands.clear();
            } else {
                single_problem_operands.push(raw_num.parse::<u64>().unwrap());
            }
        }
        if !single_problem_operands.is_empty() {
            operands.push(single_problem_operands);
        }

        (operands, operations)
    }

    fn fill_cephalopod_data(
        data: std::io::Lines<BufReader<File>>,
    ) -> (Vec<Vec<char>>, Vec<OperationType>) {
        let mut fragmented_data: Vec<Vec<char>> = vec![];

        let mut operations: Vec<OperationType> = vec![];
        for line in data.map_while(Result::ok) {
            if line.contains('+') || line.contains('*') {
                operations = line
                    .split_whitespace()
                    .rev()
                    .map(OperationType::from)
                    .collect();
            }
            let fragmented = line.replace(' ', "X").chars().collect::<Vec<char>>();
            fragmented_data.push(fragmented);
        }

        (fragmented_data, operations)
    }
}
