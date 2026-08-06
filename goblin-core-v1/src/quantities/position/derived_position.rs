use goblin_macros::FixedDecode;

use crate::quantities::{bits_layout::BitsLayout, inner_val::InnerVal, Position};
use core::ops::RangeInclusive;

#[derive(Clone, Copy, PartialEq, PartialOrd, Default, FixedDecode)]
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
    pub fn new(inner: K) -> Self {
        Self { inner }
    }

    pub fn min() -> Self {
        Self::new(K::from_u64(0))
    }

    pub fn max() -> Self {
        Self::new(K::from_u64(BitsLayout::<BITS>::MAX))
    }

    pub fn convert_range(value: RangeInclusive<Position>) -> RangeInclusive<Self> {
        let (start, end) = value.into_inner();
        RangeInclusive::new(start.into(), end.into())
    }
}

impl<const BITS: u16> DerivedPosition<u8, BITS> {
    pub fn byte_index(&self) -> usize {
        self.inner as usize / 8
    }

    pub fn bit_index(&self) -> usize {
        self.inner as usize % 8
    }
}

impl<K, const BITS: u16> From<Position> for DerivedPosition<K, BITS>
where
    K: InnerVal,
{
    fn from(value: Position) -> Self {
        let extracted = Position::extract::<BITS>(&value);
        Self::new(K::from_u64(extracted.inner))
    }
}

impl<K, const BITS: u16> From<DerivedPosition<K, BITS>> for Position
where
    K: InnerVal,
{
    fn from(value: DerivedPosition<K, BITS>) -> Self {
        let inner: u64 = value.inner.into();
        Position {
            inner: (inner & BitsLayout::<BITS>::MASK) << BitsLayout::<BITS>::OFFSET,
        }
    }
}
