mod polygon;
// use polygon::{Neighbours, Coord2d};
use crate::file_reader::FileLineIterator;
use crate::utils::part_output;
use polygon::{Coord2d, Edge};

use std::cmp::max;
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

fn get_edge_hash(sorted_coords: &Vec<Coord2d>) -> (EdgeHash, EdgeHash) {
    let mut x_lookup: HashMap<u32, Vec<usize>> = HashMap::new();
    let mut y_lookup: HashMap<u32, Vec<usize>> = HashMap::new();

    for (coord_idx, coord) in sorted_coords.iter().enumerate() {
        x_lookup.entry(coord.x).or_default().push(coord_idx);
        y_lookup.entry(coord.y).or_default().push(coord_idx);
    }

    let populate_edge_hash = |orth_lookup: &HashMap<u32, Vec<usize>>,
                              parrallel_lookup: &HashMap<u32, Vec<usize>>,
                              get_parr_axis: fn(&Coord2d) -> u32|
     -> EdgeHash {
        let mut orth_edges: EdgeHash = EdgeHash::new();

        for cur_idxs in orth_lookup.values() {
            if cur_idxs.len() < 2 {
                println!("This was not supposed to happen");
                continue;
            }
            let local_edge = Edge::new(&sorted_coords[cur_idxs[0]], &sorted_coords[cur_idxs[1]]);

            for &op_val in parrallel_lookup.keys() {
                if op_val < get_parr_axis(local_edge.start_coord)
                    || op_val > get_parr_axis(local_edge.end_coord)
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



fn run_part2(coords: &mut Vec<Coord2d>) -> Result<u64, ()> {
    let mut max_area: u64 = 0u64;

    coords.sort();

    let (orth_edges_for_x, orth_edge_for_y) = get_edge_hash(coords);

    for start_index in 0..coords.len() {
        for end_index in (start_index + 1)..coords.len() {
            let start_coord = &coords[start_index];
            let end_coord = &coords[end_index];
        }
    }

    Ok(max_area)
}

pub fn run(part: u8) -> String {
    let lines = FileLineIterator::new("src/day_09/input.txt").unwrap();
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
