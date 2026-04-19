use crate::quantities::{inner_val::InnerVal, Position};
use core::ops::RangeInclusive;

#[derive(Clone, Copy, PartialEq, PartialOrd, Default)]
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
    pub const BIT_OFFSET: u16 = BITS >> 8;
    pub const BIT_COUNT: u16 = BITS & 0b1111;

    /// Unshifted bitmask of BIT_COUNT ones
    /// e.g. BIT_COUNT=4 → 0b1111
    /// Note: not legal for BIT_COUNT = 64
    pub const MASK: u64 = (1 << Self::BIT_COUNT) - 1;

    pub const MIN_RAW: u64 = 0;
    pub const MAX_RAW: u64 = Self::MASK;

    pub fn new(inner: K) -> Self {
        Self { inner }
    }

    pub fn min() -> Self {
        Self::new(K::from_u64(Self::MIN_RAW))
    }

    pub fn max() -> Self {
        Self::new(K::from_u64(Self::MAX_RAW))
    }

    pub fn step_interval() -> usize {
        let derived_position = DerivedPosition::<u64, BITS>::new(1);
        let position = Position::from(derived_position);
        position.inner as usize
    }

    pub fn convert_range(value: RangeInclusive<Position>) -> RangeInclusive<Self> {
        let (start, end) = value.into_inner();
        RangeInclusive::new(start.into(), end.into())
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
            inner: (inner & DerivedPosition::<K, BITS>::MASK)
                << DerivedPosition::<K, BITS>::BIT_OFFSET,
        }
    }
}
