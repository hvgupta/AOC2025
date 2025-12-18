use crate::file_reader::FileLineIterator;
use crate::utils::part_output;

pub fn run(part: u8) -> String {
    let lines = FileLineIterator::new("src/day_02/input.txt").unwrap();
    let first_line = &lines.lines()[0];

    let ranges = first_line
        .split(',')
        .map(|r| {
            let mut range = r.split('-');

            let (start, end) = (range.next().unwrap(), range.next().unwrap());
            (start.to_string(), end.to_string())
        })
        .collect::<Vec<(String, String)>>();

    return match part {
        1 => part_output(run_part1, part, &ranges),
        2 => part_output(run_part2, part, &ranges),
        _ => "Invalid part number".to_string(),
    };
}
fn conv_first_nth_to_num(s: &str, until: usize) -> u64 {
    s.chars()
        .take(until)
        .fold(0u64, |acc, c| acc * 10 + c.to_digit(10).unwrap() as u64)
}

fn count_num_repeats(start: &String, end: &String, repeats: usize) -> u64 {
    let mut id_sum: u64 = 0;

    let start_first_half_digits = conv_first_nth_to_num(start, start.len() / repeats);
    let end_first_half_digits = conv_first_nth_to_num(end, (end.len() / repeats) + 1);

    let start_num = start.parse::<u64>().unwrap();
    let end_num = end.parse::<u64>().unwrap();

    for number in start_first_half_digits..=end_first_half_digits {
        let number_str = number.to_string().repeat(repeats);
        if number_str.len() == 0 {
            continue;
        }

        let number_val = match number_str.parse::<u64>(){
            Ok(val) => val,
            Err(_) => continue,
        };

        if number_val < start_num || number_val > end_num {
            continue;
        }

        id_sum += number_val;
    }

    id_sum
}

fn run_part1(ranges: &Vec<(String, String)>) -> Result<u64, ()> {
    // Implement the logic for part 1 here
    let mut id_sum: u64 = 0;

    for (start, end) in ranges {
        id_sum += count_num_repeats(start, end, 2);
    }
    Ok(id_sum)
}

fn is_invalid_id(num: u64) -> bool {
    let s = num.to_string();
    let len = s.len();
    
    // Try all possible pattern lengths
    for pattern_len in 1..=len/2 {
        if len % pattern_len != 0 {
            continue;
        }
        
        let pattern = &s[0..pattern_len];
        let repeats = len / pattern_len;
        
        // Check if whole string is pattern repeated
        if pattern.repeat(repeats) == s {
            return true;
        }
    }
    
    false
}

fn run_part2(ranges: &Vec<(String, String)>) -> Result<u64, ()> {
    let mut sum: u64 = 0;
    
    for (start_str, end_str) in ranges {
        let start: u64 = start_str.parse().unwrap();
        let end: u64 = end_str.parse().unwrap();
        
        for num in start..=end {
            if is_invalid_id(num) {
                sum += num;
            }
        }
    }
    
    Ok(sum)
}