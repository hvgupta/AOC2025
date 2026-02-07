use crate::file_reader::FileLineIterator;

use std::cmp::{Ordering, Reverse};
use std::collections::{BinaryHeap, HashMap, HashSet};

struct OrderedFloat(f32);

impl PartialEq for OrderedFloat {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl Eq for OrderedFloat {}

impl PartialOrd for OrderedFloat {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl Ord for OrderedFloat {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.partial_cmp(&other.0).unwrap_or(Ordering::Equal)
    }
}

struct Coord3d {
    x: u32,
    y: u32,
    z: u32,
}
impl Coord3d {
    pub fn new(x: u32, y: u32, z: u32) -> Self {
        Coord3d { x, y, z }
    }

    pub fn euclidean_distance(&self, other: &Coord3d) -> f32 {
        let dx = (self.x as i32 - other.x as i32) as f32;
        let dy = (self.y as i32 - other.y as i32) as f32;
        let dz = (self.z as i32 - other.z as i32) as f32;
        (dx * dx + dy * dy + dz * dz).sqrt()
    }
}

fn find(parent: &mut Vec<u16>, i: u16) -> u16 {
    if parent[i as usize] != i {
        parent[i as usize] = find(parent, parent[i as usize]);
    }
    parent[i as usize]
}

fn run_part2(
    coords: &Vec<Coord3d>,
    distance_min_heap: &mut BinaryHeap<(Reverse<OrderedFloat>, u16, u16)>,
    parent: &mut Vec<u16>,
) -> Result<u64, ()> {

    let mut current_len = parent.len() as u16;
    let mut last_edge: Option<(u16, u16)> = None;

    // Continue until all nodes are in the same component
    while current_len > 1 {
        let Some((Reverse(OrderedFloat(_dist)), i, j)) = distance_min_heap.pop() else {
            break; // No more edges to process; break to avoid infinite loop
        };

        let mut parent_copy = parent.clone(); // Create a copy for find calls
        let root_i = find(&mut parent_copy, i);
        let root_j = find(&mut parent_copy, j);
        
        if root_i != root_j {
            // Actually connect the components
            parent[root_j as usize] = root_i;
            
            // Store the coordinates of this edge (the last one we added)
            last_edge = Some((i, j));
            current_len -= 1;
        }
    }

    // Get the coordinates of the last edge that completed the MST
    if let Some((i, j)) = last_edge {
        Ok((coords[i as usize].x as u64) * (coords[j as usize].x as u64))
    } else {
        Err(())
    }
}


fn run_part1(
    distance_min_heap: &mut BinaryHeap<(Reverse<OrderedFloat>, u16, u16)>,
    parent: &mut Vec<u16>,
) -> Result<u64, ()> {
    let mut top_3_size_mult: u64 = 1;

    for _ in 0..1000 {
        if let Some((Reverse(OrderedFloat(_dist)), i, j)) = distance_min_heap.pop() {
            let root_i = find(parent, i);
            let root_j = find(parent, j);
            if root_i != root_j {
                parent[root_j as usize] = root_i;
            }
        }
    }

    let mut group_sizes: HashMap<u16, u64> = HashMap::new();
    for i in 0..parent.len() {
        let root = find(parent, i as u16);
        *group_sizes.entry(root).or_insert(0) += 1;
    }

    let mut unique_sorted_sizes: Vec<u64> = group_sizes
        .values()
        .copied()
        .collect::<HashSet<u64>>()
        .into_iter()
        .collect();

    unique_sorted_sizes.sort_by(|a, b| b.cmp(a));

    for size in unique_sorted_sizes.iter().take(3) {
        top_3_size_mult *= *size;
    }
    Ok(top_3_size_mult)
}

pub fn run(part: u8) -> String {
    let lines = FileLineIterator::new("src/day_08/input.txt").unwrap();
    let coords = lines
        .lines()
        .iter()
        .map(|f| {
            let parts: Vec<&str> = f.split(',').collect();
            Coord3d::new(
                parts[0].parse::<u32>().unwrap(),
                parts[1].parse::<u32>().unwrap(),
                parts[2].parse::<u32>().unwrap(),
            )
        })
        .collect::<Vec<Coord3d>>();

    let mut distance_min_heap: BinaryHeap<(Reverse<OrderedFloat>, u16, u16)> = BinaryHeap::new();

    for i in 0..coords.len() {
        for j in (i + 1)..coords.len() {
            let dist = coords[i].euclidean_distance(&coords[j]);
            distance_min_heap.push((Reverse(OrderedFloat(dist)), i as u16, j as u16));
        }
    }

    let mut parent: Vec<u16> = (0..(coords.len() as u16)).collect();

    return match part {
        1 => run_part1(&mut distance_min_heap, &mut parent)
            .unwrap()
            .to_string(),
        2 => run_part2(&coords, &mut distance_min_heap, &mut parent)
            .unwrap()
            .to_string(),
        _ => "Invalid part number".to_string(),
    };
}
