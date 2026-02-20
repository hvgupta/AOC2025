#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Coord2d {
    pub x: u32,
    pub y: u32,
}
impl Coord2d {
    pub fn new(x: u32, y: u32) -> Self {
        Coord2d { x, y }
    }

    pub fn from_string(coord_string: &String) -> Self {
        let mut splitted_string = coord_string.split(",");
        let x: u32 = splitted_string.next().unwrap().parse().unwrap();
        let y: u32 = splitted_string.next().unwrap().parse().unwrap();
        Coord2d::new(x, y)
    }

    pub fn calc_area(&self, other: &Coord2d) -> u64 {
        let x_dist_mult = (self.x as i64 - other.x as i64).abs() as u64 + 1u64;
        let y_dist_mult = (self.y as i64 - other.y as i64).abs() as u64 + 1u64;
        x_dist_mult * y_dist_mult
    }

    // pub fn set_neighbours(&mut self, neighbours: Neighbours) {
    //     self.neighbours = neighbours;
    // }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Dir {
    DOWN,
    RIGHT,
}

#[derive(Debug, Clone, Copy)]
pub struct Edge<'a> {
    pub start_coord: &'a Coord2d,
    pub end_coord: &'a Coord2d,
    pub dir: Dir,
}
impl<'a> Edge<'a> {
    pub fn new(start_coord: &'a Coord2d, end_coord: &'a Coord2d) -> Edge<'a> {
        if start_coord.x == end_coord.x {
            let min_coord = if start_coord.y < end_coord.y {
                start_coord
            } else {
                end_coord
            };
            let max_coord = if min_coord == start_coord {
                end_coord
            } else {
                start_coord
            };

            Edge {
                start_coord: min_coord,
                end_coord: max_coord,
                dir: Dir::DOWN,
            }
        } else {
            let min_coord = if start_coord.x < end_coord.x {
                start_coord
            } else {
                end_coord
            };
            let max_coord = if min_coord == start_coord {
                end_coord
            } else {
                start_coord
            };
            Edge {
                start_coord: min_coord,
                end_coord: max_coord,
                dir: Dir::RIGHT,
            }
        }
    }
    
    pub fn intersects(&self, other: &Edge) -> bool {
        if self.dir == other.dir {
            return false;
        }

        let right_edge = if self.dir == Dir::RIGHT { self } else { other };
        let down_edge = if self.dir == Dir::DOWN { self } else { other };

        (right_edge.start_coord.x..right_edge.end_coord.x).contains(&down_edge.start_coord.x)
            && (down_edge.start_coord.y..down_edge.end_coord.y).contains(&right_edge.start_coord.y)
    }
}
