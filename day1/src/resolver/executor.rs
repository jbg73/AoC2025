use std::cmp::Ordering;

use crate::parser::data_parser::{DialStep, Direction};

pub struct Executor {}

impl Executor {
    pub fn rotate(current: i32, dial_step: &DialStep) -> i32 {
        let result = match dial_step.dir {
            Direction::Left => Executor::left(current, dial_step.distance),
            Direction::Right => Executor::right(current, dial_step.distance),
            _ => current,
        };

        result
    }

    fn left(current: i32, distance: i32) -> i32 {
        let normalized_distance = distance % 100;

        let result = match PartialOrd::partial_cmp(&current, &normalized_distance) {
            Some(Ordering::Equal) | Some(Ordering::Greater) => current - normalized_distance,
            Some(Ordering::Less) => 100 - (normalized_distance - current),
            None => current,
        };

        result
    }

    fn right(current: i32, distance: i32) -> i32 {
        let normalized_distance = distance % 100;

        let result = match PartialOrd::partial_cmp(&(99 - &current), &normalized_distance) {
            Some(Ordering::Equal) | Some(Ordering::Greater) => current + normalized_distance,
            Some(Ordering::Less) => normalized_distance - (100 - current),
            None => current,
        };

        result
    }
}
