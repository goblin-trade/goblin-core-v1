use core::ops::RangeInclusive;
use core::ops::{Add, Sub};
use core::u64;

use crate::axis::leg::leg_matcher::LegMatcher;
use crate::quantities::DerivedPosition;

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
    /// # Safety
    ///
    /// Externally ensure that `offset` is ≥ 63
    ///
    pub fn complement<const BITS: u16>(&self) -> Self {
        let offset = BITS >> 8;
        let mask = !((1u64 << (offset + 1)) - 1);
        Self {
            inner: self.inner & mask,
        }
    }

    /// Extracts the bitfield defined by `BITS` and returns it as a `Position`
    /// with all other bits zeroed.
    pub fn extract<const BITS: u16>(&self) -> Self {
        let inner = (self.inner >> DerivedPosition::<u64, BITS>::BIT_OFFSET)
            & DerivedPosition::<u64, BITS>::MASK;

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
