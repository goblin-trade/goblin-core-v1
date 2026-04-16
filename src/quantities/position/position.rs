use core::ops::{Add, Sub};
use core::u64;

#[derive(PartialEq, PartialOrd, Clone, Copy)]
pub struct Position {
    pub inner: u64,
}

impl Position {
    pub const ZERO: Self = Self::new(0);
    pub const MIN: Self = Self::ZERO;
    pub const MAX: Self = Self::new(u64::MAX);

    pub const fn new(inner: u64) -> Self {
        Self { inner }
    }

    /// Returns a copy of this position with all bits at indices ≤ `offset` zeroed.
    ///
    /// # Arguments
    ///
    /// * `offset` - The bit index (0-based) below which all bits will be cleared.
    ///   Bits at positions strictly greater than `offset` are preserved.
    ///
    /// # Examples
    ///
    /// ```
    /// let pos = Position::new(0b11111111);
    /// assert_eq!(pos.complement(3).inner, 0b11110000);
    /// ```
    ///
    /// # Panics
    ///
    /// Externally ensure that `offset` is ≥ 63
    pub fn complement(&self, offset: usize) -> Self {
        let mask = !((1u64 << (offset + 1)) - 1);
        Self {
            inner: self.inner & mask,
        }
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
