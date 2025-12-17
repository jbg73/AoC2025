use std::cmp::Ordering;

use crate::parser::data_parser::{DialStep, Direction};

pub struct RotationResult {
    pub position: i32,
    pub zero_clicks: i32,
}

pub struct Executor {}

impl Executor {
    pub fn rotate(current: i32, dial_step: &DialStep) -> RotationResult {
        let result = match dial_step.dir {
            Direction::Left => Executor::left(current, dial_step.distance),
            Direction::Right => Executor::right(current, dial_step.distance),
            _ => RotationResult {
                position: current,
                zero_clicks: 0,
            },
        };

        result
    }

    fn left(current: i32, distance: i32) -> RotationResult {
        let full_turns = distance / 100;
        let mut zero_clicks = full_turns;
        let normalized_distance = distance % 100;

        let new_position = match PartialOrd::partial_cmp(&current, &normalized_distance) {
            Some(Ordering::Equal) | Some(Ordering::Greater) => current - normalized_distance,
            Some(Ordering::Less) => 100 - (normalized_distance - current),
            None => current,
        };

        if (current > 0 && new_position > current) || new_position == 0 {
            println!("Passing zero rotating left!");
            zero_clicks = zero_clicks + 1;
        }

        RotationResult {
            position: new_position,
            zero_clicks,
        }
    }

    fn right(current: i32, distance: i32) -> RotationResult {
        let full_turns = distance / 100;
        let mut zero_clicks = full_turns;
        let normalized_distance = distance % 100;

        let new_position = match PartialOrd::partial_cmp(&(99 - &current), &normalized_distance) {
            Some(Ordering::Equal) | Some(Ordering::Greater) => current + normalized_distance,
            Some(Ordering::Less) => normalized_distance - (100 - current),
            None => current,
        };

        if (current > 0 && new_position < current) || new_position == 0 {
            println!("Passing zero rotating right!");
            zero_clicks = zero_clicks + 1;
        }

        RotationResult {
            position: new_position,
            zero_clicks,
        }
    }
}
