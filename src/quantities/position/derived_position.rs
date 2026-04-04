use crate::quantities::Position;

pub struct DerivedPosition<K, const BIT_OFFSET: usize, const BIT_COUNT: usize>
where
    K: From<u64> + Into<u64>,
{
    pub inner: K,
}

impl<K, const BIT_OFFSET: usize, const BIT_COUNT: usize> DerivedPosition<K, BIT_OFFSET, BIT_COUNT>
where
    K: From<u64> + Into<u64>,
{
    /// Unshifted bitmask of BIT_COUNT ones
    /// e.g. BIT_COUNT=4 → 0b1111
    /// Note: not legal for BIT_COUNT = 64
    pub const MASK: u64 = (1 << BIT_COUNT) - 1;

    pub const MIN: u64 = 0;
    pub const MAX: u64 = Self::MASK;

    #[inline]
    pub fn new(inner: K) -> Self {
        Self { inner }
    }
}

impl<K, const BIT_OFFSET: usize, const BIT_COUNT: usize> From<Position>
    for DerivedPosition<K, BIT_OFFSET, BIT_COUNT>
where
    K: From<u64> + Into<u64>,
{
    fn from(value: Position) -> Self {
        let extracted = (value.inner >> BIT_OFFSET) & Self::MASK;

        Self {
            inner: extracted.into(),
        }
    }
}

impl<K, const BIT_OFFSET: usize, const BIT_COUNT: usize>
    From<DerivedPosition<K, BIT_OFFSET, BIT_COUNT>> for Position
where
    K: From<u64> + Into<u64>,
{
    fn from(value: DerivedPosition<K, BIT_OFFSET, BIT_COUNT>) -> Self {
        let inner: u64 = value.inner.into();

        Position {
            inner: (inner & DerivedPosition::<K, BIT_OFFSET, BIT_COUNT>::MASK) << BIT_OFFSET,
        }
    }
}
