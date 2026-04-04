use crate::quantities::{inner_val::InnerVal, Position};

#[derive(Clone, Copy)]
pub struct DerivedPosition<K, const BIT_OFFSET: usize, const BIT_COUNT: usize>
where
    K: InnerVal,
{
    pub inner: K,
}

impl<K, const BIT_OFFSET: usize, const BIT_COUNT: usize> DerivedPosition<K, BIT_OFFSET, BIT_COUNT>
where
    K: InnerVal,
{
    /// Unshifted bitmask of BIT_COUNT ones
    /// e.g. BIT_COUNT=4 → 0b1111
    /// Note: not legal for BIT_COUNT = 64
    pub const MASK: u64 = (1 << BIT_COUNT) - 1;

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
}

impl<K, const BIT_OFFSET: usize, const BIT_COUNT: usize> From<Position>
    for DerivedPosition<K, BIT_OFFSET, BIT_COUNT>
where
    K: InnerVal,
{
    fn from(value: Position) -> Self {
        let extracted = (value.inner >> BIT_OFFSET) & Self::MASK;
        Self::new(K::from_u64(extracted))
    }
}

impl<K, const BIT_OFFSET: usize, const BIT_COUNT: usize>
    From<DerivedPosition<K, BIT_OFFSET, BIT_COUNT>> for Position
where
    K: InnerVal,
{
    fn from(value: DerivedPosition<K, BIT_OFFSET, BIT_COUNT>) -> Self {
        let inner: u64 = value.inner.into();

        Position {
            inner: (inner & DerivedPosition::<K, BIT_OFFSET, BIT_COUNT>::MASK) << BIT_OFFSET,
        }
    }
}
