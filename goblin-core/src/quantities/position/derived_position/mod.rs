use goblin_macros::FixedDecode;

use crate::quantities::{Position, inner_val::InnerVal};
use core::range::RangeInclusive;

pub mod bitmap_position;
pub mod full_position;

pub use bitmap_position::*;
pub use full_position::*;
mod impl_add;
mod impl_from;

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
            start: value.start.into(),
            last: value.last.into(),
        }
    }
}
