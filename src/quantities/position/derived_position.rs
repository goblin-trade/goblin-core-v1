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
        // Mask is not legal for BIT_COUNT = 64
        let mask = (1u64 << BIT_COUNT) - 1;
        let extracted = (value.inner >> BIT_OFFSET) & mask;

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
        let mask = (1u64 << BIT_COUNT) - 1;
        let inner: u64 = value.inner.into();

        Position {
            inner: (inner & mask) << BIT_OFFSET,
        }
    }
}
