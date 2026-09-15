use crate::quantities::{BitsLayout, DerivedPosition, InnerVal, Position};

impl<K, const BITS: u16> From<Position> for DerivedPosition<K, BITS>
where
    K: InnerVal,
{
    fn from(value: Position) -> Self {
        let extracted = Position::extract::<BITS>(&value);
        Self::new(K::truncate_from_u64(extracted.inner))
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
