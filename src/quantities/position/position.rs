use core::ops::{Add, Sub};

#[derive(PartialEq, PartialOrd, Clone, Copy)]
pub struct Position {
    pub inner: u64,
}

impl Position {
    pub const fn new(inner: u64) -> Self {
        Self { inner }
    }
}

impl Add for Position {
    type Output = Position;

    fn add(self, rhs: Position) -> Self::Output {
        Position {
            inner: self.inner + rhs.inner,
        }
    }
}

impl Sub for Position {
    type Output = Position;

    fn sub(self, rhs: Position) -> Self::Output {
        Position {
            inner: self.inner - rhs.inner,
        }
    }
}
