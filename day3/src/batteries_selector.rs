use std::cmp::Ordering;

pub struct BatterySelector {}

impl BatterySelector {
    pub fn select_battery(battery_bank: &str) -> u32 {
        let battery_bank_digits = BatterySelector::convert_to_digits(battery_bank);
        let left_battery = BatterySelector::left_selector(&battery_bank_digits);
        let right_battery = BatterySelector::right_selector(&battery_bank_digits, left_battery.1);

        left_battery.0 * 10 + right_battery
    }

    fn convert_to_digits(battery_bank: &str) -> Vec<u32> {
        // todo: use From trait
        let digits: Vec<u32> = battery_bank
            .chars()
            .into_iter()
            .map(|b| b.to_digit(10).unwrap())
            .collect();

        digits
    }

    fn left_selector(battery_bank: &Vec<u32>) -> (u32, usize) {
        let max_found = battery_bank
            .iter()
            .take(battery_bank.len() - 1)
            .max()
            .unwrap()
            .clone();
        let position = battery_bank.iter().position(|&x| x == max_found).unwrap();

        (max_found, position)
    }

    fn right_selector(battery_bank: &Vec<u32>, left_constraint: usize) -> u32 {
        let max_found = battery_bank
            .clone()
            .into_iter()
            .rev()
            .take(battery_bank.len() - left_constraint - 1)
            .max()
            .unwrap();

        max_found
    }
}
