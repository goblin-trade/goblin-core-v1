use core::ops::{Add, AddAssign, Sub, SubAssign};

use crate::quantities::{Exp, Quantity};

/// Blanket trait for all supported Quantity operations
///
pub trait QuantityOps:
    Copy + Sized + PartialEq + Default + Add<Output = Self> + Sub<Output = Self> + AddAssign + SubAssign
{
    const MIN: Self;
    const MAX: Self;
    const ZERO: Self;
    const ONE: Self;

    fn checked_add(self, rhs: Self) -> Option<Self>;
    fn checked_sub(self, rhs: Self) -> Option<Self>;
}

// Implementation for constants, addition and subtraction.
// These will be used in the leg namespace.
//
// Multiplication and division operations are asymmetric and happen
// in the side namespace.
impl<D> QuantityOps for Quantity<D>
where
    D: Exp + Copy + PartialEq + Default,
{
    const MIN: Self = Self::new(u64::MIN);
    const MAX: Self = Self::new(u64::MAX);
    const ZERO: Self = Self::new(0);
    const ONE: Self = Self::new(1);

    fn checked_add(self, rhs: Self) -> Option<Self> {
        self.inner.checked_add(rhs.inner).map(Quantity::new)
    }

    fn checked_sub(self, rhs: Self) -> Option<Self> {
        self.inner.checked_sub(rhs.inner).map(Quantity::new)
    }
}
