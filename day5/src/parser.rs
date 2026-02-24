use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use crate::inventory::{Ingredient, Range};

pub struct InputParser {}

impl InputParser {
    pub fn read_data(filename: &str) -> (Vec<Range>, Vec<Ingredient>) {
        let file = File::open(filename).expect("Error while opening the file!");
        let reader = BufReader::new(file);

        let mut ingredient_ranges: Vec<Range> = vec![];
        let mut available_ingredients: Vec<Ingredient> = vec![];
        reader.lines().for_each(|l| {
            let read_line = l.unwrap();
            if read_line.find('-').is_some() {
                ingredient_ranges.push(Range::from(read_line.as_str()));
            } else if !read_line.is_empty() {
                available_ingredients.push(Ingredient::from(read_line.as_str()))
            }
        });

        (ingredient_ranges, available_ingredients)
    }
}
