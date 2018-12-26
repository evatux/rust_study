// u-data types

use std::ops::Add;
use std::ops::Sub;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Pos {
    pub x: u16,
    pub y: u16,
}

pub type Board = Pos;

impl Add for Pos {
    type Output = Pos;

    fn add(self, other: Pos) -> Pos {
        Pos { x: self.x + other.x, y: self.y + other.y }
    }
}

impl Sub for Pos {
    type Output = Pos;

    fn sub(self, other: Pos) -> Pos {
        Pos { x: self.x - other.x, y: self.y - other.y }
    }
}
