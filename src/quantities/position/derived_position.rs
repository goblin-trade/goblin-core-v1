use crate::quantities::Position;
use core::usize;

pub struct DerivedPosition<K, const BIT_OFFSET: usize, const BIT_COUNT: usize>(pub K)
where
    K: From<u64> + Into<u64>;

impl<K, const BIT_OFFSET: usize, const BIT_COUNT: usize> From<Position>
    for DerivedPosition<K, BIT_OFFSET, BIT_COUNT>
where
    K: From<u64> + Into<u64>,
{
    fn from(value: Position) -> Self {
        // Mask is not legal for BIT_COUNT=64
        let mask = (1 << BIT_COUNT) - 1;
        let extracted = (value.0 >> BIT_OFFSET) & mask;
        Self(extracted.into())
    }
}

impl<K, const BIT_OFFSET: usize, const BIT_COUNT: usize>
    From<DerivedPosition<K, BIT_OFFSET, BIT_COUNT>> for Position
where
    K: From<u64> + Into<u64>,
{
    fn from(value: DerivedPosition<K, BIT_OFFSET, BIT_COUNT>) -> Self {
        let mask: u64 = (1u64 << BIT_COUNT) - 1;
        let inner: u64 = value.0.into();
        Position((inner & mask) << BIT_OFFSET)
    }
}
