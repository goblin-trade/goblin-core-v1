use core::ops::{Add, Sub};

#[derive(PartialEq, PartialOrd)]
pub struct Position(pub u64);

impl Add for Position {
    type Output = Position;

    fn add(self, rhs: Position) -> Self::Output {
        Position(self.0 + rhs.0)
    }
}

impl Sub for Position {
    type Output = Position;

    fn sub(self, rhs: Position) -> Self::Output {
        Position(self.0 - rhs.0)
    }
}
