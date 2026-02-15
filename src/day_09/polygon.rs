// #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Default)]
// pub struct Neighbours {
//     pub right: Option<usize>,
//     pub down: Option<usize>,
// }
// impl Neighbours {
//     pub fn count(&self) -> u8 {
//         let mut num_neighbours = 0;
//         num_neighbours += (self.down != None) as u8;
//         num_neighbours += (self.right != None) as u8;
//         num_neighbours
//     }
// }

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Coord2d {
    pub x: u32,
    pub y: u32,
    // pub neighbours: Neighbours,
}
impl Coord2d {
    pub fn new(x: u32, y: u32) -> Self {
        Coord2d {
            x,
            y,
            // neighbours: Neighbours {
            //     right: None,
            //     down: None,
            // },
        }
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
enum Dir {
    DOWN,
    RIGHT,
}

#[derive(Debug, Clone, Copy)]
pub struct Edge<'a> {
    pub start_coord: &'a Coord2d,
    pub end_coord: &'a Coord2d,
    dir: Dir,
}
impl<'a> Edge<'a> {
    pub fn new(start_coord: &'a Coord2d, end_coord: &'a Coord2d) -> Edge<'a> {
        Edge {
            start_coord,
            end_coord,
            dir: if start_coord.x == end_coord.x {
                Dir::DOWN
            } else {
                Dir::RIGHT
            },
        }
    }

    pub fn get_common_val(&self) ->&u32{
        if self.dir == Dir::DOWN {
            &self.start_coord.x
        } else {
            &self.start_coord.y
        }
    }

    pub fn intersects(&self, other: &Edge)->bool{
        if self.dir == other.dir {
            return false;
        }

        self.start_coord.x < other.start_coord.x 
            && self.end_coord.x > other.start_coord.x 
            && self.start_coord.y > other.start_coord.y
            && self.start_coord.y < other.end_coord.y 
    }
}
