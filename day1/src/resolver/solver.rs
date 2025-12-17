use crate::{parser::data_parser::DialStep, resolver::Executor};

pub struct Solver {
    current: i32,
    steps: Vec<DialStep>,
}

impl Solver {
    pub fn new(start: i32, steps: Vec<DialStep>) -> Self {
        Self {
            current: start,
            steps,
        }
    }

    pub fn solve(&mut self) -> i32 {
        self.steps.iter().fold(0, |acc, e| {
            self.current = Executor::rotate(self.current, e);

            if self.current == 0 { acc + 1 } else { acc + 0 }
        })
    }
}
