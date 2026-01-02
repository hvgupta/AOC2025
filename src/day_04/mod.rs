use crate::file_reader::FileLineIterator;
use crate::utils::part_output;

const DIRS : [(i32, i32); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

pub fn run(part: u8) -> String {
    let lines = FileLineIterator::new("src/day_04/input.txt").unwrap();

    return match part {
        1 => part_output(run_part1, part, &lines),
        2 => part_output(run_part2, part, &lines),
        _ => "Invalid part number".to_string(),
    };
}



fn run_part1(file: &FileLineIterator) -> Result<u32, ()> {
    let mut total_removable_rolls: u32 = 0;

    let grid = file.lines();

    for (row_index, line) in grid.iter().enumerate() {
        for (col_index, cell) in line.chars().enumerate() {
            if cell == '.' {
                continue;
            }
            
            if count_surrounding_rolls(grid, row_index, line, col_index) < 4 {
                total_removable_rolls += 1;
            }
        }
    }

    Ok(total_removable_rolls)
}

fn count_surrounding_rolls(grid: &Vec<String>, row_index: usize, line: &String, col_index: usize) -> i32 {
    let mut surrounding_rolls = 0;
    for dir in DIRS.iter() {
        let new_x = row_index as i32 + dir.0;
        let new_y = col_index as i32 + dir.1;

        if new_x < 0 || new_y < 0 || new_x >= grid.len() as i32 || new_y >= line.len() as i32 {
            continue;
        }

        let adjacent_cell = grid[new_x as usize].chars().nth(new_y as usize).unwrap();
        // println!("Adjacent Cell at ({}, {}): {}", new_x, new_y, adjacent_cell);
        if adjacent_cell == '@' {
            surrounding_rolls += 1;
        }
    }
    surrounding_rolls
}

fn grid_iterate(grid: &Vec<String>) -> (u32, Vec<String>) {
    let mut total_removable_rolls: u32 = 0;
    let mut new_grid = grid.clone();

    for (row_index, line) in grid.iter().enumerate() {
        for (col_index, cell) in line.chars().enumerate() {
            if cell == '.' {
                continue;
            }      
            
            if count_surrounding_rolls(grid, row_index, line, col_index) < 4 {
                total_removable_rolls += 1;
                // Mark this cell as removed in the new grid
                let mut new_line_chars: Vec<char> = new_grid[row_index].chars().collect();
                new_line_chars[col_index] = '.';
                new_grid[row_index] = new_line_chars.into_iter().collect();
            }
        }
    }

    (total_removable_rolls, new_grid)
}

fn run_part2(file: &FileLineIterator) -> Result<u32, ()> {
    let mut total_removable_rolls: u32 = 0;

    let mut grid = file.lines().clone();

    loop {
        let (removed_rolls, new_grid) = grid_iterate(&grid);
        if removed_rolls == 0 {
            break;
        }
        total_removable_rolls += removed_rolls;
        grid = new_grid;
    }

    Ok(total_removable_rolls)
}