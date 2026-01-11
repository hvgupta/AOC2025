use crate::file_reader::FileLineIterator;
use crate::utils::part_output;

use std::collections::HashMap;
use std::collections::HashSet;

pub fn run(part: u8) -> String {
    let lines = FileLineIterator::new("src/day_07/input.txt").unwrap();

    return match part {
        1 => part_output(run_part1, part, &lines),
        2 => part_output(run_part2, part, &lines),
        _ => "Invalid part number".to_string(),
    };
}

fn conv_line_splitter_index(line: &String) -> HashSet<u32> {
    let mut split_indices = HashSet::<u32>::new();

    line.char_indices().for_each(|(idx, c)| {
        if c == '^' {
            split_indices.insert(idx as u32);
        }
    });

    split_indices
}

fn num_splits_at_cur_line(cur_tachyons: &mut HashSet<u32>, line: &String) -> u32 {
    let splitter_indices = conv_line_splitter_index(line);
    let line_max_len = line.len() as u32;

    let mut new_tachyons = HashSet::<u32>::new();
    let mut num_splits: u32 = 0;

    for splitter_index in splitter_indices {
        if !cur_tachyons.contains(&splitter_index) {
            continue;
        }
        if splitter_index > 0 {
            new_tachyons.insert(splitter_index - 1);
        }
        if splitter_index < (line_max_len) - 1 {
            new_tachyons.insert(splitter_index + 1);
        }

        cur_tachyons.remove(&splitter_index);

        num_splits += 1;
    }

    new_tachyons.extend(cur_tachyons.iter());

    *cur_tachyons = new_tachyons;

    num_splits
}

fn initialization(line: &String) -> HashSet<u32> {
    let mut cur_tachyons = HashSet::<u32>::new();
    cur_tachyons.insert(line.find('S').unwrap() as u32);
    cur_tachyons
}

fn run_part1(lines: &FileLineIterator) -> Result<u32, ()> {
    let mut num_splits: u32 = 0;
    let _cur_tachyons: HashSet<u32> = HashSet::<u32>::new();

    let mut cur_tachyons = initialization(&lines.lines().get(0).unwrap());

    for (line_index, line) in lines.lines().iter().enumerate().skip(1) {
        if line_index % 2 == 1 {
            continue;
        }

        num_splits += num_splits_at_cur_line(&mut cur_tachyons, line);
    }

    Ok(num_splits)
}

fn calc_num_timelines_at_line(prev_index_timelines: &mut HashMap<u32, u64>, lines: &String) -> HashMap<u32, u64> {
    let mut cur_index_timelines = HashMap::<u32, u64>::new();

    let cur_splitter_indices = conv_line_splitter_index(lines);
    for splitter_index in cur_splitter_indices {
        if !prev_index_timelines.contains_key(&splitter_index) {
            continue;
        }
        let timelines_at_splitter = prev_index_timelines.get(&splitter_index).unwrap();
        if splitter_index > 0 {
            *cur_index_timelines.entry(splitter_index - 1).or_insert(0) += timelines_at_splitter;
        }
        if splitter_index < (lines.len() as u32) - 1 {
            *cur_index_timelines.entry(splitter_index + 1).or_insert(0) += timelines_at_splitter;
        }

        prev_index_timelines.remove(&splitter_index);
    }

    for (idx, timelines) in prev_index_timelines.iter() {
        *cur_index_timelines.entry(*idx).or_insert(0) += timelines; 
    }

    cur_index_timelines
}

fn run_part2(lines: &FileLineIterator) -> Result<u64, ()> {
    let mut index_timelines = HashMap::<u32, u64>::new();
    index_timelines.insert(lines.lines().get(0).unwrap().find('S').unwrap() as u32, 1);

    for (line_index, line) in lines.lines().iter().enumerate().skip(1) {
        if line_index % 2 == 1 {
            continue;
        }

        index_timelines = calc_num_timelines_at_line(&mut index_timelines, line);
    }

    let total_timelines = index_timelines.values().sum();

    Ok(total_timelines)
}
