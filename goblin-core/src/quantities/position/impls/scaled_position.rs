use crate::quantities::{InnerVal, Position};

pub trait ScaledPosition<K: InnerVal, const BITS: u16> {
    fn scale_up(&self) -> Position<u64, BITS>;
}

impl<K: InnerVal, const BITS: u16> ScaledPosition<K, BITS> for Position<K, BITS> {
    fn scale_up(&self) -> Position<u64, BITS> {
        Position {
            inner: self.inner.into(),
        }
    }
}
