mod alias;
pub mod bits_layout;
pub mod derived_position;
mod from_ticks;
pub mod inner_val;
pub mod position_range;
pub mod safe_position;

pub use alias::*;
pub use bits_layout::*;
pub use derived_position::*;
pub use inner_val::*;
pub use position_range::*;
pub use safe_position::*;

use goblin_macros::FixedDecode;

use core::ops::{Add, Sub};

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug, FixedDecode)]
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
    /// # Safety
    ///
    /// Externally ensure that `offset` is ≥ 63
    ///
    pub fn complement<const BITS: u16>(&self) -> Self {
        let mask = !((1u64 << (BitsLayout::<BITS>::OFFSET + 1)) - 1);
        Self {
            inner: self.inner & mask,
        }
    }

    /// Extracts the bitfield defined by `BITS` and returns it as a `Position`
    /// with all other bits zeroed.
    pub fn extract<const BITS: u16>(&self) -> Self {
        let inner = (self.inner >> BitsLayout::<BITS>::OFFSET) & BitsLayout::<BITS>::MASK;
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
