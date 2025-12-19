use crate::file_reader::FileLineIterator;
use crate::utils::part_output;

use std::cmp::max;

pub fn run(part: u8) -> String {
    let lines = FileLineIterator::new("src/day_03/input.txt").unwrap();

    return match part {
        1 => part_output(run_part1, part, &lines),
        2 => part_output(run_part2, part, &lines),
        _ => "Invalid part number".to_string(),
    };
}

fn run_part1(lines: &FileLineIterator) -> Result<u32, ()> {
    let mut total_joltage: u32 = 0;

    for bank in lines.lines() {
        let mut left_battery_value = bank.chars().nth(0).unwrap().to_digit(10).unwrap();

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

fn largest_k_digit_subsequence(num_str: &String) -> String {
    let m = num_str.len();

    if 12 >= m {
        return num_str.to_string();
    }

    let mut stack = Vec::with_capacity(m);
    let mut to_remove = m - 12; 

    for ch in num_str.chars() {
        while !stack.is_empty() && to_remove > 0 {
            let last = *stack.last().unwrap();
            if last < ch {
                stack.pop();
                to_remove -= 1;
            } else {
                break;
            }
        }
        stack.push(ch);
    }

    
    if to_remove > 0 {
        stack.truncate(stack.len() - to_remove);
    }

    
    stack.iter().take(12).collect()
}

fn run_part2(lines: &FileLineIterator) -> Result<u64, ()> {
    let mut total_joltage: u64 = 0;

    for bank in lines.lines() {
        let largest_subseq = largest_k_digit_subsequence(bank);
        let joltage = match largest_subseq.parse::<u64>() {
            Ok(val) => val,
            Err(_) => continue,
        };
        total_joltage += joltage;

    }
    Ok(total_joltage)
}
