use crate::file_reader::FileLineIterator;
use crate::utils::part_output;

fn create_p1_table(lines: &FileLineIterator) -> Vec<Vec<String>> {
    let mut table: Vec<Vec<String>> = Vec::new();

    for line in lines.lines() {
        let row = line
            .split_whitespace()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        table.push(row);
    }

    table
}

pub fn run(part: u8) -> String {
    let lines = FileLineIterator::new("src/day_06/input.txt").unwrap();

    return match part {
        1 => part_output(run_part1, part, &lines),
        2 => part_output(run_part2, part, &lines),
        _ => "Invalid part number".to_string(),
    };
}

fn run_part1(lines: &FileLineIterator) -> Result<u64, ()> {
    let mut hw_grand_total = 0;
    let table = create_p1_table(lines);

    for col in 0..table[0].len() {
        let cur_op_is_add = table.last().unwrap()[col] == "+";
        let mut col_value = if cur_op_is_add { 0 } else { 1 };
        for row in 0..table.len() - 1 {
            let cell_value = table[row][col].parse::<u64>().unwrap();
            if cur_op_is_add {
                col_value += cell_value;
            } else {
                col_value *= cell_value;
            }
        }
        hw_grand_total += col_value;
    }

    Ok(hw_grand_total)
}

fn create_p2_table(lines: &FileLineIterator) -> Vec<Vec<String>> {
    let mut table = Vec::<Vec<String>>::new();

    // the simplier solution is to use regex, but I want to try make a manual parser

    let hw_lines = lines.lines();
    let operator_line = hw_lines.last().unwrap();

    let mut op_index = Vec::<usize>::new();

    for (index, op) in operator_line.chars().enumerate() {
        if op == '+' || op == '*' {
            op_index.push(index);
        }
    }

    for line in hw_lines.iter().take(hw_lines.len() - 1) {
        let mut row = Vec::<String>::new();
        let op_index_iterator = op_index.iter();

        for (idx, op_idx) in op_index_iterator.take(op_index.len() - 1).enumerate() {
            let cell = &line[*op_idx..op_index[idx + 1]-1];
            row.push(cell.to_string());
        }

        row.push((&line[*(op_index.last().unwrap())..]).to_string());
        table.push(row);
    }

    table.push(operator_line.split_whitespace().map(|s| s.to_string()).collect());

    table
}

fn conv_vertical_to_horizontal_numbers(col: &Vec<String>) -> Vec<u64> {
    let num_digits = col[0].len();
    let mut horizontal_numbers = Vec::<u64>::with_capacity(num_digits);

    for digit_idx in 0..num_digits {
        let mut digit_str = String::new();
        for row in col.iter().take(col.len() - 1) {
            digit_str.push(row.chars().nth(digit_idx).unwrap());
        }
        horizontal_numbers.push(digit_str.trim().parse::<u64>().unwrap());
    }

    horizontal_numbers
}

fn run_part2(lines: &FileLineIterator) -> Result<u64, ()> {
    let mut hw_grand_total = 0;

    let table = create_p2_table(lines);

    for col in 0..table[0].len() {
        let cur_op_is_add = table.last().unwrap()[col] == "+";
        let col_vertical = table.iter().map(|row| row[col].to_string()).collect::<Vec<String>>();
        let horizontal_numbers = conv_vertical_to_horizontal_numbers(&col_vertical);

        let mut col_value = if cur_op_is_add { 0 } else { 1 };
        for number in horizontal_numbers.iter() {
            if cur_op_is_add {
                col_value += *number;
            } else {
                col_value *= *number;
            }
        }
        hw_grand_total += col_value;
    }

    

    Ok(hw_grand_total)
}
