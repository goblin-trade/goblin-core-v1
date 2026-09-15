use goblin_macros::FixedDecode;

use crate::quantities::{BitsLayout, Position, inner_val::InnerVal};
use core::range::RangeInclusive;

pub mod bitmap_position;
pub mod full_position;

pub use bitmap_position::*;
pub use full_position::*;
mod impl_add;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, FixedDecode)]
pub struct DerivedPosition<K, const BITS: u16>
where
    K: InnerVal,
{
    pub inner: K,
}

impl<K, const BITS: u16> DerivedPosition<K, BITS>
where
    K: InnerVal,
{
    pub const fn new(inner: K) -> Self {
        Self { inner }
    }

    pub fn convert_range(value: RangeInclusive<Position>) -> RangeInclusive<Self> {
        RangeInclusive {
            start: value.start.extract_and_convert(),
            last: value.last.extract_and_convert(),
        }
    }

    pub fn into_position(&self) -> Position {
        let inner: u64 = self.inner.into();
        Position {
            inner: (inner & BitsLayout::<BITS>::MASK) << BitsLayout::<BITS>::OFFSET,
        }
    }
}
