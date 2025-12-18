use crate::file_reader::FileLineIterator;
use crate::utils::part_output;

use std::cmp::max;

pub fn run(part: u8) -> String {
    let lines = FileLineIterator::new("src/day_03/input.txt").unwrap();

    return match part {
        1 => part_output(run_part1, part, &lines),
        // 2 => "",
        _ => "Invalid part number".to_string(),
    };
}

fn run_part1(lines: &FileLineIterator) -> Result<u32, ()> {
    let mut total_joltage: u32 = 0;

    for bank in lines.lines() {
        let mut left_battery_value = bank
            .chars()
            .nth(0)
            .unwrap()
            .to_digit(10)
            .unwrap();

        let mut cur_max = 0;

        for battery in bank.chars().skip(1) {
            let right_battery_value = battery.to_digit(10).unwrap();

            cur_max = max(cur_max, left_battery_value * 10 + right_battery_value);

            if right_battery_value > left_battery_value {
                left_battery_value = right_battery_value;
            }
        }

        total_joltage += cur_max;
    }

    Ok(total_joltage)
}
