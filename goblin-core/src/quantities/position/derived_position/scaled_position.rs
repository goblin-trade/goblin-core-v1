use crate::quantities::{DerivedPosition, InnerVal};

pub trait ScaledPosition<K: InnerVal, const BITS: u16> {
    fn scale_up(&self) -> DerivedPosition<u64, BITS>;
}

impl<K: InnerVal, const BITS: u16> ScaledPosition<K, BITS> for DerivedPosition<K, BITS> {
    fn scale_up(&self) -> DerivedPosition<u64, BITS> {
        DerivedPosition {
            inner: self.inner.into(),
        }
    }
}
