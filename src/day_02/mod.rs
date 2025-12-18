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

    println!("{:?}", ranges);

    return match part {
        1 => part_output(run_part1, part, &ranges),
        // 2 => part_output(run_part2, part, &lines),
        _ => "Invalid part number".to_string(),
    };
}

fn run_part1(ranges: &Vec<(String, String)>) -> Result<u64, ()> {
    // Implement the logic for part 1 here
    let mut id_sum: u64 = 0;

    fn conv_first_half_to_num(s: &str, until: usize) -> u64 {
        s.chars()
            .take(until)
            .fold(0u64, |acc, c| acc * 10 + c.to_digit(10).unwrap() as u64)
    }

    for (start, end) in ranges {
        let start_num = start.parse::<u64>().unwrap();
        let end_num = end.parse::<u64>().unwrap();

        let start_first_half_digits = conv_first_half_to_num(start, start.len() / 2);
        let end_first_half_digits = conv_first_half_to_num(end, (end.len() / 2) + 1);

        for number in start_first_half_digits..=end_first_half_digits {
            let number_str = number.to_string().repeat(2);
            if number_str.len() < start.len() {
                continue;
            }

            let number_val = number_str.parse::<u64>().unwrap();

            if number_val < start_num || number_val > end_num {
                continue;
            }

            println!("Valid number: {}", number_val);

            id_sum += number_val;
        }
    }
    Ok(id_sum)
}
