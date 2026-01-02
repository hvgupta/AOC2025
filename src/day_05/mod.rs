use crate::file_reader::FileLineIterator;
use crate::utils::part_output;

use std::cmp::max;

fn get_inputs(file: &FileLineIterator) -> (Vec<(u64, u64)>, Vec<u64>) {
    let mut ranges = Vec::<(u64, u64)>::new();
    let mut ingredients = Vec::<u64>::new();

    let mut range_mode = true;

    for line in file.lines() {
        if line.is_empty() {
            range_mode = false;
            continue;
        }
        if range_mode {
            let range_str = line.split("-").collect::<Vec<&str>>();
            ranges.push((
                range_str[0].parse::<u64>().unwrap(),
                range_str[1].parse::<u64>().unwrap(),
            ));
        } else {
            ingredients.push(line.parse::<u64>().unwrap());
        }
    }

    ranges.sort();
    ingredients.sort();

    let mut fresh_ingredients_list = Vec::<(u64, u64)>::new();
    let mut current_range = ranges[0];
    for range in ranges.iter().skip(1) {
        if range.0 <= current_range.1 + 1 {
            current_range.1 = max(range.1, current_range.1);
        } else {
            fresh_ingredients_list.push(current_range);
            current_range = *range;
        }
    }

    fresh_ingredients_list.push(current_range);

    (fresh_ingredients_list, ingredients)
}

pub fn run(part: u8) -> String {
    let lines = FileLineIterator::new("src/day_05/input.txt").unwrap();

    let input = get_inputs(&lines);

    return match part {
        1 => part_output(run_part1, part, &input),
        2 => part_output(run_part2, part, &input.0),
        _ => "Invalid part number".to_string(),
    };
}

fn run_part1(
    (fresh_ingredients_list, ingredients): &(Vec<(u64, u64)>, Vec<u64>),
) -> Result<usize, ()> {
    let mut num_fresh_ingredients = 0;
    let mut l: usize = 0;
    for ingredient in ingredients.iter() {
        let mut r: usize = fresh_ingredients_list.len();
        loop {
            if l >= r {
                break;
            }
            let mid = (l + r) / 2;
            let range = &fresh_ingredients_list[mid];

            if range.0 <= *ingredient && *ingredient <= range.1 {
                num_fresh_ingredients += 1;
                break;
            } else if ingredient < &range.0 {
                r = mid;
            } else {
                l = mid + 1;
            }
        }
    }

    Ok(num_fresh_ingredients)
}

fn run_part2(ranges: &Vec<(u64, u64)>) -> Result<u64, ()> {
    let mut total_covered = 0;
    for range in ranges.iter() {
        total_covered += range.1 - range.0 + 1;
    }
    Ok(total_covered)
}
