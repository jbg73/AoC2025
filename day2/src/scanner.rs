use crate::parser::data_parser::Range;

pub struct Scanner {}

impl Scanner {
    pub fn scan(range: &Range) -> i64 {
        let sum_invalid_ids = (range.0..=range.1).into_iter().fold(0, |acc, id| {
            if Scanner::is_invalid(&id) {
                // println!("Adding {}", id);
                // println!("Current acc {} \n", acc + id);
                acc + id
            } else {
                acc + 0
            }
        });

        sum_invalid_ids
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
}
