use crate::quantities::{Exp, Quantity};

impl<E> Quantity<E, u32>
where
    E: Exp,
{
    /// Widen to the 64 bit equivalent.
    ///
    /// `const` so 32 bit wire values can be widened during const evaluation,
    /// e.g. when building hardcoded market definitions.
    pub const fn widen_to_u64(self) -> Quantity<E, u64> {
        Quantity::new(self.inner as u64)
    }
}

/// Widen a 32 bit quantity to its 64 bit equivalent.
///
/// 32 bit quantities are used in wire headers to reduce encoded size. Internal
/// math is 64 bit, so header values are widened after decoding.
impl<E> From<Quantity<E, u32>> for Quantity<E, u64>
where
    E: Exp,
{
    fn from(value: Quantity<E, u32>) -> Self {
        value.widen_to_u64()
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
