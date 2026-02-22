use std::ops::Range;

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
}

#[derive(Clone, Copy)]
pub enum Dir {
    INCREASING,
    DECREASING,
}

#[derive(Debug, Clone, Copy)]
pub enum Edge<'a> {
    Horizontal {
        start_coord: &'a Coord2d,
        end_coord: &'a Coord2d,
    },
    Vertical {
        start_coord: &'a Coord2d,
        end_coord: &'a Coord2d,
    },
}

impl<'a> Edge<'a> {
    pub fn new(a: &'a Coord2d, b: &'a Coord2d) -> Result<Self, &'static str> {
        if a.x == b.x {
            if a.y < b.y {
                Ok(Edge::Vertical {
                    start_coord: a,
                    end_coord: b,
                })
            } else {
                Ok(Edge::Vertical {
                    start_coord: b,
                    end_coord: a,
                })
            }
        } else if a.y == b.y {
            if a.x < b.x {
                Ok(Edge::Horizontal {
                    start_coord: a,
                    end_coord: b,
                })
            } else {
                Ok(Edge::Horizontal {
                    start_coord: b,
                    end_coord: a,
                })
            }
        } else {
            Err("only axis-aligned edges are supported")
        }
    }

    fn get_range(start: u32, end: u32, invalid_dir: Dir) -> Range<u32> {
        match invalid_dir {
            Dir::INCREASING => start..end,
            Dir::DECREASING => (start + 1)..(end + 1),
        }
    }

    pub fn intersects(&self, other: &Edge, invalid_direction: Dir) -> bool {
        let (h0, h1, v0, v1) = match (self, other) {
            (
                Edge::Horizontal {
                    start_coord: h0,
                    end_coord: h1,
                },
                Edge::Vertical {
                    start_coord: v0,
                    end_coord: v1,
                },
            ) => {
                if h0.x == v0.x || h0.x == v1.x {
                    return false;
                }
                if h1.x == v0.x || h1.x == v1.x {
                    return false;
                }
                (h0, h1, v0, v1)
            }
            (
                Edge::Vertical {
                    start_coord: v0,
                    end_coord: v1,
                },
                Edge::Horizontal {
                    start_coord: h0,
                    end_coord: h1,
                },
            ) => {
                if v0.y == h0.y || v0.y == h1.y {
                    return false;
                }
                if v1.y == h0.y || v1.y == h1.y {
                    return false;
                }
                (h0, h1, v0, v1)
            }
            _ => return false,
        };
        
        let y = h0.y;
        let x0 = h0.x;
        let x1 = h1.x;

        let x = v0.x;
        let y0 = v0.y;
        let y1 = v1.y;
        let x_range = Edge::get_range(x0, x1, invalid_direction);
        let y_range = Edge::get_range(y0, y1, invalid_direction);

        x_range.contains(&x) && y_range.contains(&y)
    }

    pub fn intersects_with_any_edges(
        &self,
        others: &Vec<Edge<'a>>,
        invalid_direction: Dir,
    ) -> bool {
        others
            .iter()
            .any(|other| self.intersects(other, invalid_direction))
    }
}
