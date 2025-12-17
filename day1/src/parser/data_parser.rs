use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
pub enum Direction {
    Left,
    Right,
    Invalid,
}

#[derive(Debug)]
pub struct DialStep {
    pub dir: Direction,
    pub distance: i32,
}

pub struct DataParser {}

impl DataParser {
    fn get_direction(dir_s: char) -> Direction {
        match dir_s {
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => Direction::Invalid,
        }
    }

    pub fn parse_data(filename: &str) -> Vec<DialStep> {
        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);

        let lines = reader.lines();

        let mut data: Vec<DialStep> = Vec::new();
        for line in lines {
            let line = line.unwrap();
            let dir: Direction = DataParser::get_direction(line.chars().nth(0).unwrap());
            let distance = line[1..].parse::<i32>().unwrap();

            data.push(DialStep { dir, distance });
        }
        data
    }
}
