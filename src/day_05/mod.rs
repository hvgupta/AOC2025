use crate::file_reader::FileLineIterator;
use crate::utils::part_output;


pub fn run(part: u8) -> String {
    let lines = FileLineIterator::new("src/day_05/input.txt").unwrap();

    return match part {
        // 1 => part_output(run_part1, part, &lines),
        // 2 => part_output(run_part2, part, &lines),
        _ => "Invalid part number".to_string(),
    };
}


fn run_part1(file: &FileLineIterator) -> Result<u32, ()> {
    Ok(0)
}