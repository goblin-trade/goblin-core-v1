use crate::quantities::{Exp, Quantity};

/// Widen a 32 bit quantity to its 64 bit equivalent.
///
/// 32 bit quantities are used in wire headers to reduce encoded size. Internal
/// math is 64 bit, so header values are widened after decoding.
impl<E> From<Quantity<E, u32>> for Quantity<E, u64>
where
    E: Exp,
{
    fn from(value: Quantity<E, u32>) -> Self {
        Self::new(value.inner.into())
    }
}

impl<E> From<Quantity<E, i32>> for Quantity<E, i64>
where
    E: Exp,
{
    fn from(value: Quantity<E, i32>) -> Self {
        Self::new(value.inner.into())
    }
}
