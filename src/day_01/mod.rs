use crate::file_reader::FileLineIterator;

pub fn run(part: u8)-> String {
    let rotations = FileLineIterator::new("src/day_01/input.txt").unwrap();
    fn part_output(function: fn(&FileLineIterator) -> Result<u32, ()>, part: u8, rotations: &FileLineIterator) -> String {
        match function(&rotations) {
            Ok(result) => format!("Part {}: {:?}", part, result),
            Err(_) => format!("Error in Part {}", part),
        }
    }

    return match part {
        1 => part_output(run_part1, part, &rotations),
        2 => part_output(run_part2, part, &rotations),
        _ => "Invalid part number".to_string(),
    };
}

fn run_part1(rotations: &FileLineIterator) -> Result<u32, ()>{
    let mut cur_num: i32 = 50;
    let mut zero_count: u32 = 0;
    for rotation in rotations.lines() {
        let (dir, num) = rotation.split_at(1);
        let num = num.parse::<i32>().unwrap();
        cur_num = match dir {
            "L" => cur_num - num,
            "R" => cur_num + num,
            _ => return Err(()),
        } % 100;

        zero_count += (cur_num == 0) as u32;
    }
    Ok(zero_count)
}

fn run_part2(rotations: &FileLineIterator) -> Result<u32, ()>{
    Ok(0)
}