use core::fmt;
use std::fmt::Debug;

#[derive(PartialEq)]
pub enum Direction {
    N,
    NE,
    E,
    SE,
    S,
    SW,
    W,
    NW,
}

#[derive(Debug, Eq, Hash, PartialEq, Copy, Clone, Ord, PartialOrd)]
pub struct Coord {
    pub x: i32,
    pub y: i32,
}

impl fmt::Display for Coord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

impl Coord {
    pub fn abs_distance(&self, other: &Coord) -> (i32, i32) {
        ((self.x - other.x).abs(), (self.y - other.y).abs())
    }

    /// The other coord's direction relative to my position
    ///
    /// For example, if the function returns North, that means the other coord is towards self's north
    pub fn relative_direction(&self, other: &Coord) -> Option<Direction> {
        let dist = self.abs_distance(other);

        if dist.0 == 0 && dist.1 == 0 {
            return None;
        }
        if dist.0 == 0 {
            if self.y < other.y {
                return Some(Direction::N);
            } else {
                return Some(Direction::S);
            }
        } else if dist.1 == 0 {
            if self.x < other.x {
                return Some(Direction::E);
            } else {
                return Some(Direction::W);
            }
        } else {
            if self.x < other.x {
                if self.y < other.y {
                    return Some(Direction::NE);
                } else {
                    return Some(Direction::SE);
                }
            } else {
                if self.y < other.y {
                    return Some(Direction::NW);
                } else {
                    return Some(Direction::SW);
                }
            }
        }
    }

    pub fn displace(&mut self, direction: Direction, step: i32) {
        match direction {
            Direction::N => {
                self.y += step;
            }
            Direction::NE => {
                self.x += step;
                self.y += step;
            }
            Direction::E => {
                self.x += step;
            }
            Direction::SE => {
                self.y -= step;
                self.x += step;
            }
            Direction::S => {
                self.y -= step;
            }
            Direction::SW => {
                self.y -= step;
                self.x -= step;
            }
            Direction::W => {
                self.x -= step;
            }
            Direction::NW => {
                self.y += step;
                self.x -= step;
            }
        }
    }
}
