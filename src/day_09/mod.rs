mod polygon;
// use polygon::{Neighbours, Coord2d};
use crate::file_reader::FileLineIterator;
use crate::utils::part_output;
use polygon::{Coord2d, Dir, Edge};

use std::cmp::{max, min};
use std::collections::HashMap;

type EdgeHash<'a> = HashMap<u32, Vec<Edge<'a>>>;

fn run_part1(coords: &Vec<Coord2d>) -> Result<u64, ()> {
    let mut max_area = 0u64;

    for start_index in 0..coords.len() {
        for end_index in (start_index + 1)..coords.len() {
            let area = coords[start_index].calc_area(&coords[end_index]);
            max_area = max(max_area, area);
        }
    }

    Ok(max_area)
}

fn get_edge_hash<'a>(sorted_coords: &'a Vec<Coord2d>) -> (EdgeHash<'a>, EdgeHash<'a>) {
    let mut x_lookup: HashMap<u32, Vec<usize>> = HashMap::new();
    let mut y_lookup: HashMap<u32, Vec<usize>> = HashMap::new();

    for (coord_idx, coord) in sorted_coords.iter().enumerate() {
        x_lookup.entry(coord.x).or_default().push(coord_idx);
        y_lookup.entry(coord.y).or_default().push(coord_idx);
    }

    let populate_edge_hash = |orth_lookup: &HashMap<u32, Vec<usize>>,
                              parrallel_lookup: &HashMap<u32, Vec<usize>>,
                              get_parr_axis: fn(&Coord2d) -> u32|
     -> EdgeHash<'a> {
        let mut orth_edges: EdgeHash = EdgeHash::new();

        for cur_idxs in orth_lookup.values() {
            if cur_idxs.len() < 2 {
                println!("This was not supposed to happen");
                continue;
            }
            let local_edge =
                Edge::new(&sorted_coords[cur_idxs[0]], &sorted_coords[cur_idxs[1]]).unwrap();

            for &op_val in parrallel_lookup.keys() {
                if op_val < get_parr_axis(&sorted_coords[cur_idxs[0]])
                    || op_val > get_parr_axis(&sorted_coords[cur_idxs[1]])
                {
                    continue;
                }
                orth_edges
                    .entry(op_val)
                    .or_default()
                    .push(local_edge.clone());
            }
        }

        orth_edges
    };

    let orth_edges_for_x =
        populate_edge_hash(&y_lookup, &x_lookup, |coord: &Coord2d| -> u32 { coord.x });

    let orth_edges_for_y =
        populate_edge_hash(&x_lookup, &y_lookup, |coord: &Coord2d| -> u32 { coord.y });

    (orth_edges_for_x, orth_edges_for_y)
}

fn is_rect_valid(
    start_coord: &Coord2d,
    end_coord: &Coord2d,
    orth_edges_for_x: &EdgeHash,
    orth_edge_for_y: &EdgeHash,
) -> bool {
    let min_x = min(start_coord.x, end_coord.x);
    let max_x = max(start_coord.x, end_coord.x);
    let min_y = min(start_coord.y, end_coord.y);
    let max_y = max(start_coord.y, end_coord.y);

    // Not a proper rectangle
    if min_x == max_x || min_y == max_y {
        return false;
    }

    let bottom_left_coord = Coord2d::new(min_x, min_y);
    let top_left_coord = Coord2d::new(min_x, max_y);
    let top_right_coord = Coord2d::new(max_x, max_y);
    let bottom_right_coord = Coord2d::new(max_x, min_y);

    let left_edge = Edge::new(&bottom_left_coord, &top_left_coord).unwrap();
    let top_edge = Edge::new(&top_left_coord, &top_right_coord).unwrap();
    let bottom_edge = Edge::new(&bottom_left_coord, &bottom_right_coord).unwrap();
    let right_edge = Edge::new(&bottom_right_coord, &top_right_coord).unwrap();

    let check_for_intersections =
        |check_edge: &Edge, orth_lookup: &EdgeHash, key: u32, invalid_dir: Dir| -> bool {
            if let Some(orth_edges) = orth_lookup.get(&key) {
                if check_edge.intersects_with_any_edges(orth_edges, invalid_dir) {
                    return true;
                }
            }
            false
        };

    if check_for_intersections(&left_edge, orth_edges_for_x, min_x, Dir::INCREASING)
        || check_for_intersections(&top_edge, orth_edge_for_y, max_y, Dir::DECREASING)
        || check_for_intersections(&right_edge, orth_edges_for_x, max_x, Dir::DECREASING)
        || check_for_intersections(&bottom_edge, orth_edge_for_y, min_y, Dir::INCREASING)
    {
        return false;
    }

    true
}

fn run_part2(coords: &mut Vec<Coord2d>) -> Result<u64, ()> {
    let mut max_area: u64 = 0u64;

    coords.sort();

    let (orth_edges_for_x, orth_edges_for_y) = get_edge_hash(coords);

    for start_index in 0..coords.len() {
        for end_index in (start_index + 1)..coords.len() {
            let start_coord = &coords[start_index];
            let end_coord = &coords[end_index];

            if !is_rect_valid(start_coord, end_coord, &orth_edges_for_x, &orth_edges_for_y) {
                continue;
            }
            max_area = max(max_area, start_coord.calc_area(end_coord));
        }
    }

    Ok(max_area)
}

pub fn run(part: u8) -> String {
    let lines = FileLineIterator::new("./src/day_09/input.txt").unwrap();
    let mut coords: Vec<Coord2d> = lines
        .lines()
        .iter()
        .map(|line| Coord2d::from_string(line))
        .collect();

    return match part {
        1 => part_output(run_part1, 1, &coords),
        2 => format!("Part 2: {:?}", run_part2(&mut coords).unwrap()),
        _ => "Invalid part number, the parts are 1 and 2".to_string(),
    };
}
