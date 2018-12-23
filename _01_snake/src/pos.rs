// Pos data type

use std::ops::Add;

#[derive(Debug, PartialEq)]
pub struct Pos(pub u16, pub u16);

impl<'a, 'b> Add<&'a Pos> for &'b Pos {
    type Output = Pos;

    fn add(self, other: &'a Pos) -> Pos {
        Pos(self.0 + other.0, self.1 + other.1)
    }
}
