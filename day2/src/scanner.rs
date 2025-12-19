use std::{collections::HashMap, vec};

use crate::parser::data_parser::Range;

pub struct Scanner {}

impl Scanner {
    pub fn scan(range: &Range) -> i64 {
        let sum_invalid_ids = (range.0..=range.1).into_iter().fold(0, |acc, id| {
            if Scanner::is_invalid_extra(&id) {
                acc + id
            } else {
                acc + 0
            }
        });

        sum_invalid_ids
    }

    fn get_relevant_multiples(number: &usize) -> Vec<usize> {
        let mut multiples: Vec<usize> = Vec::new();
        for m in 1..=number / 2 {
            if number % m == 0 {
                multiples.push(m);
            }
        }

        multiples
    }

    fn is_invalid(id: &i64) -> bool {
        let id_parsed = id.to_string();
        let id_size = id_parsed.len();
        if id_size % 2 != 0 {
            return false;
        }

        let first_half = &id_parsed[0..id_size / 2]; // todo: why removing & doesn't work?
        let second_half = &id_parsed[id_size / 2..id_size];

        first_half == second_half
    }

    fn is_invalid_extra(id: &i64) -> bool {
        let id_parsed = id.to_string();
        let id_size = id_parsed.len();

        let multiples = Scanner::get_relevant_multiples(&id_size);

        let mut is_valid = false;
        for m in multiples {
            let pattern = &id_parsed[..m];
            is_valid = Scanner::is_pattern_repeated(pattern, &id_parsed);
            if is_valid {
                break;
            }
        }

        is_valid
    }

    fn is_pattern_repeated(pattern: &str, id: &str) -> bool {
        let id_size = id.len();
        let pattern_size = pattern.len();

        let mut is_valid = true;
        let mut count = 0;
        while count < id_size {
            let next = &id[count..count + pattern_size];
            if next != pattern {
                is_valid = false;
                break;
            }
            count = count + pattern_size;
        }

        is_valid
    }
}
