use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug)]
pub struct Range(pub i64, pub i64);

pub struct DataParser {}

impl DataParser {
    pub fn parse_data(filename: &str) -> Vec<Range> {
        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);
        let lines = reader.lines();

        let mut data: Vec<Range> = Vec::new();
        for line in lines {
            let line = line.unwrap();
            let ranges = line.split(',');
            for range in ranges {
                let start = range.split('-').nth(0).unwrap().parse::<i64>().unwrap();
                let end = range.split('-').nth(1).unwrap().parse::<i64>().unwrap();
                data.push(Range(start, end));
            }
        }

        data
    }
}
