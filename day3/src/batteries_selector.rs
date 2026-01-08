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

    fn left_selector(battery_bank: &[u32]) -> (u32, usize) {
        let max_found = battery_bank
            .iter()
            .take(battery_bank.len())
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

    pub fn max_joltage_selector(battery_bank: &str, n_batteries: usize) -> u64 {
        let mut max_joltage_bank: String = String::new();

        let mut current_pos = 0;
        while max_joltage_bank.len() < n_batteries {
            let safety_battery_idx = battery_bank.len() - (n_batteries - max_joltage_bank.len());
            let battery_block_digits =
                Self::convert_to_digits(&battery_bank[current_pos..=safety_battery_idx]);
            let next_max_joltage = Self::left_selector(&battery_block_digits[..]);
            max_joltage_bank = max_joltage_bank + next_max_joltage.0.to_string().as_str();

            current_pos = current_pos + next_max_joltage.1 + 1;
        }

        max_joltage_bank.parse::<u64>().unwrap()
    }
}
