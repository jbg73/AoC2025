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
            println!("Step: {:?}", e);
            let res = Executor::rotate(self.current, e);
            self.current = res.position;
            println!("Current clicks: {}", acc + res.zero_clicks);
            println!("New position: {}\n", self.current);

            acc + res.zero_clicks
        })
    }
}
