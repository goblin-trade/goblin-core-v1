use core::ops::Add;

use crate::quantities::{DerivedPosition, InnerVal};

/// Add two DerivedPosition types
///
/// # Safety
///
/// We only add Pos0 into Pos1 and Pos1 into Pos2. This cannot overflow.
impl<K: InnerVal, const BITS: u16> Add for DerivedPosition<K, BITS> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            inner: self.inner + rhs.inner,
        }
    }
}
