mod alias;
mod impls;

pub mod bits_layout;
pub mod inner_val;
pub mod position_range;
pub mod safe_position;

pub use alias::*;
pub use bits_layout::*;
pub use impls::*;
pub use inner_val::*;
pub use position_range::*;
pub use safe_position::*;

use core::range::RangeInclusive;

use goblin_macros::FixedDecode;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, FixedDecode, Default)]
pub struct Position<K, const BITS: u16>
where
    K: InnerVal,
{
    pub inner: K,
}

impl<K, const BITS: u16> Position<K, BITS>
where
    K: InnerVal,
{
    pub const fn new(inner: K) -> Self {
        Self { inner }
    }

    pub fn convert_range(value: RangeInclusive<FullPos>) -> RangeInclusive<Self> {
        RangeInclusive {
            start: value.start.extract_and_convert(),
            last: value.last.extract_and_convert(),
        }
    }

    pub fn into_position(&self) -> FullPos {
        let inner: u64 = self.inner.into();
        FullPos {
            inner: (inner & BitsLayout::<BITS>::MASK) << BitsLayout::<BITS>::OFFSET,
        }
    }
}
