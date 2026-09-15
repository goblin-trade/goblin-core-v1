use crate::quantities::{BitsLayout, DerivedPosition, InnerVal, PositionV2};

/// Transformations for DerivedPosition<u64, POS_2>
pub trait FullPosition {
    /// Returns a copy of this position with all bits at indices ≤ `offset` zeroed.
    ///
    /// # Safety
    ///
    /// Externally ensure that `offset` is ≥ 63
    ///
    fn complement<const BITS: u16>(&self) -> Self;

    /// Return a new Position instance with the extracted bits
    fn extract<const BITS: u16>(&self) -> Self;

    /// Return a new DerivedPosition instance with the extracted bits and given size
    fn extract_and_convert<K: InnerVal, const BITS: u16>(&self) -> DerivedPosition<K, BITS>;
}

impl FullPosition for PositionV2 {
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

    fn extract_and_convert<K: InnerVal, const BITS: u16>(&self) -> DerivedPosition<K, BITS> {
        let extracted = self.extract::<BITS>();
        DerivedPosition {
            inner: K::from_u64(extracted.inner),
        }
    }
}
