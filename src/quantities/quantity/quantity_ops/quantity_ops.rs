use core::ops::{Add, AddAssign, Sub, SubAssign};

use crate::{
    quantities::{Exp, Quantity},
    settlement::{CheckedAdd, ConstZero},
};

/// Blanket trait for all supported Quantity operations
///
pub trait QuantityOps:
    Copy
    + Sized
    + PartialEq
    + Default
    + Add<Output = Self>
    + Sub<Output = Self>
    + AddAssign
    + SubAssign
    + PartialOrd
    + Ord
{
    const MIN: Self;
    const MAX: Self;
    const ZERO: Self;
    const ONE: Self;

    fn checked_sub(self, rhs: Self) -> Option<Self>;
}

// Implementation for constants, addition and subtraction.
// These will be used in the leg namespace.
//
// Multiplication and division operations are asymmetric and happen
// in the side namespace.
impl<D> QuantityOps for Quantity<D>
where
    D: Exp + Copy + PartialEq + Default + PartialOrd + Ord,
{
    const MIN: Self = Self::new(u64::MIN);
    const MAX: Self = Self::new(u64::MAX);
    const ZERO: Self = Self::new(0);
    const ONE: Self = Self::new(1);

    fn checked_sub(self, rhs: Self) -> Option<Self> {
        self.inner.checked_sub(rhs.inner).map(Quantity::new)
    }
}

impl<D> ConstZero for Quantity<D>
where
    D: Exp + Copy + PartialEq + Default + PartialOrd + Ord,
{
    const ZEROED: Self = Self::ZERO;
}

impl<D> CheckedAdd for Quantity<D>
where
    D: Exp + Copy + PartialEq + Default + PartialOrd + Ord,
{
    fn checked_add(self, rhs: Self) -> Option<Self> {
        self.inner.checked_add(rhs.inner).map(Quantity::new)
    }
}
