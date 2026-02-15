mod polygon;
// use polygon::{Neighbours, Coord2d};
use crate::file_reader::FileLineIterator;
use crate::utils::part_output;
use polygon::{Coord2d, Edge};

use std::cmp::max;
use std::collections::HashMap;

type EdgeHash<'a> = HashMap<u32, Vec<Edge<'a>>>;

fn get_edges<'a>(coords: &'a Vec<Coord2d>) -> (EdgeHash<'a>, EdgeHash<'a>, Vec<Edge<'a>>) {
    let mut x_lookup: HashMap<u32, Vec<usize>> = HashMap::new();
    let mut y_lookup: HashMap<u32, Vec<usize>> = HashMap::new();

    // Group coordinates
    for (idx, coord) in coords.iter().enumerate() {
        x_lookup.entry(coord.x).or_default().push(idx);
        y_lookup.entry(coord.y).or_default().push(idx);
    }

    let mut orth_edges_for_x: HashMap<u32, Vec<Edge>> = HashMap::new();
    let mut orth_edges_for_y: HashMap<u32, Vec<Edge>> = HashMap::new();
    let mut all_edges: Vec<Edge> = Vec::new();

    let all_y_keys: Vec<u32> = y_lookup.keys().cloned().collect();
    let all_x_keys: Vec<u32> = x_lookup.keys().cloned().collect();

    for idxs in x_lookup.values() { // y edges
        if idxs.len() < 2 {
            continue;
        }

        let start_coord = &coords[idxs[0]];
        let end_coord = &coords[idxs[1]];

        let edge: Edge<'a> = Edge::new(start_coord, end_coord); // y edge (parallel to y axis)

        for &y_val in &all_y_keys {
            // for all the y coorinates in the boundary coordinates
            if y_val <= start_coord.y || y_val >= end_coord.y { // intersection at endpoint does not matter
                continue;
            }
            orth_edges_for_y // since we want to insrt a y-axis and then get y axis parrel edges, this does that
                .entry(y_val)
                .or_default()
                .push(edge.clone());
        }
        all_edges.push(edge.clone());
    }

    for idxs in y_lookup.values() {
        if idxs.len() < 2 {
            continue;
        }

        let start_coord = &coords[idxs[0]];
        let end_coord = &coords[idxs[1]];

        let edge: Edge<'a> = Edge::new(start_coord, end_coord); // Create edge once

        for &x_val in &all_x_keys {
            if x_val <= start_coord.x || x_val >= end_coord.x {
                continue;
            }
            orth_edges_for_x
                .entry(x_val)
                .or_default()
                .push(edge.clone());
        }
    }

    (orth_edges_for_x, orth_edges_for_y, all_edges)
}

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

fn check_for_intersect(local_edge: &Edge, orth_edges: &Vec<Edge>) -> bool {
    for orth_edge in orth_edges {
        if local_edge.intersects(orth_edge) {
            return true;
        }
    }

    false
}

fn rect_is_valid(
    start_coord: &Coord2d,
    end_coord: &Coord2d,
    orth_edges_for_x: &HashMap<u32, Vec<Edge>>,
    orth_edges_for_y: &HashMap<u32, Vec<Edge>>,
) -> bool {
    let bottom_left = Coord2d::new(start_coord.x, end_coord.y);
    let top_right = Coord2d::new(end_coord.x, start_coord.y);

    let left_edge = Edge::new(start_coord, &bottom_left);
    let top_edge = Edge::new(start_coord, &top_right);
    let right_edge = Edge::new(&top_right, end_coord);
    let bottom_edge = Edge::new(&bottom_left, end_coord);

    if let Some(edges_crossing_left) = orth_edges_for_x.get(&left_edge.get_common_val()) {
        if check_for_intersect(&left_edge, edges_crossing_left) {
            return false;
        }
    }

    if let Some(edges_crossing_top) = orth_edges_for_y.get(&top_edge.get_common_val()) {
        if check_for_intersect(&top_edge, edges_crossing_top) {
            return false;
        }
    }

    if let Some(edges_crossing_right) = orth_edges_for_x.get(&right_edge.get_common_val()) {
        if check_for_intersect(&right_edge, edges_crossing_right) {
            return false;
        }
    }

    if let Some(edges_crossing_bottom) = orth_edges_for_y.get(&bottom_edge.get_common_val()) {
        if check_for_intersect(&bottom_edge, edges_crossing_bottom) {
            return false;
        }
    }

    true
}

fn is_inside_polygon(mid_point: &Coord2d, max_coord: &Coord2d, orth_edges: &Vec<Edge>) -> bool {
    let ray_cast = Edge::new(mid_point, max_coord);

    let mut num_intersects = 0u16;
    for orth_edge in orth_edges {
        if !ray_cast.intersects(orth_edge) {
            continue;
        }
        num_intersects += 1;
    }

    (num_intersects % 2) != 0
}

fn run_part2(coords: &Vec<Coord2d>) -> Result<u64, ()> {
    let mut max_area: u64 = 0u64;
    let max_coord_x = coords.iter().map(|c| c.x).max().unwrap_or(0u32);

    let (orth_edges_for_x, orth_edges_for_y, all_edges) = get_edges(coords);

    for start_idx in 0..coords.len() {
        let start_coord = &coords[start_idx];
        for end_idx in start_idx + 1..coords.len() {
            let end_coord = &coords[end_idx];
            if !rect_is_valid(start_coord, end_coord, &orth_edges_for_x, &orth_edges_for_y) {
                continue;
            }

            let mid_x = (start_coord.x + end_coord.x) / 2;
            let mid_y = (start_coord.y + end_coord.y) / 2;
            let mid = Coord2d::new(mid_x, mid_y);
            let local_max_coord = Coord2d::new(max_coord_x + 1, mid.y);

            if !is_inside_polygon(&mid, &local_max_coord, &all_edges) {
                continue;
            }

            max_area = max(max_area, coords[start_idx].calc_area(&coords[end_idx]));
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

    coords.sort();
    return match part {
        1 => part_output(run_part1, 1, &coords),
        2 => format!("Part 2: {:?}", run_part2(&coords).unwrap()),
        _ => "Invalid part number, the parts are 1 and 2".to_string(),
    };
}
