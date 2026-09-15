use crate::quantities::{BitsLayout, FullPos, InnerVal, Position};

/// Transformations for Position<u64, POS_2>
pub trait FullPosition {
    const ZERO: Self;
    const MAX: Self;

    /// Returns a copy of this position with all bits at indices ≤ `offset` zeroed.
    ///
    /// # Safety
    ///
    /// Externally ensure that `offset` is ≥ 63
    ///
    fn complement<const BITS: u16>(&self) -> Self;

    /// Return a new FullPos instance with the extracted bits
    fn extract<const BITS: u16>(&self) -> Self;

    /// Return a new Position instance with the extracted bits and given size
    fn extract_and_convert<K: InnerVal, const BITS: u16>(&self) -> Position<K, BITS>;
}

impl FullPosition for FullPos {
    const ZERO: Self = Self::new(u64::MIN);
    const MAX: Self = Self::new(u64::MAX);

    fn complement<const BITS: u16>(&self) -> Self {
        let mask = !((1u64 << (BitsLayout::<BITS>::OFFSET + 1)) - 1);
        Self {
            inner: self.inner & mask,
        }
    }

    fn extract<const BITS: u16>(&self) -> Self {
        let inner = (self.inner >> BitsLayout::<BITS>::OFFSET) & BitsLayout::<BITS>::MASK;
        Self { inner }
    }

    fn extract_and_convert<K: InnerVal, const BITS: u16>(&self) -> Position<K, BITS> {
        let extracted = self.extract::<BITS>();
        Position {
            inner: K::truncate_from_u64(extracted.inner),
        }
    }
}
