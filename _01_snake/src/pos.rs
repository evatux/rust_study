// Pos data type

use std::ops::Add;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Pos(pub u16, pub u16);

impl Add for Pos {
    type Output = Pos;

    fn add(self, other: Pos) -> Pos {
        Pos(self.0 + other.0, self.1 + other.1)
    }
}
