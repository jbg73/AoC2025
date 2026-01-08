use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub struct DataParser {}

impl DataParser {
    pub fn parse_data(filename: &str) -> Vec<String> {
        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);

        let lines = reader.lines();
        let mut data: Vec<String> = Vec::new();
        for line in lines {
            let line = line.unwrap();
            data.push(line);
        }

        data
    }
}
