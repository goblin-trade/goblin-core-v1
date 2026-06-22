use core::ops::{Add, AddAssign, Sub, SubAssign};

use crate::{
    quantities::{Exp, Quantity},
    settlement::{CheckedOps, ConstZero},
};

/// Blanket trait for all supported Quantity operations
///
/// Hack- implement QuantityOps on i64 and u64 for clean API
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
    + ConstZero
    + CheckedOps
{
    const MIN: Self;
    const MAX: Self;
    const ONE: Self;
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
    const ONE: Self = Self::new(1);
}

impl<D> ConstZero for Quantity<D>
where
    D: Exp + Copy + PartialEq + Default + PartialOrd + Ord,
{
    const ZEROED: Self = Self::new(0);
}

impl<D> CheckedOps for Quantity<D>
where
    D: Exp + Copy + PartialEq + Default + PartialOrd + Ord,
{
    fn checked_add(self, rhs: Self) -> Option<Self> {
        self.inner.checked_add(rhs.inner).map(Quantity::new)
    }

    fn checked_sub(self, rhs: Self) -> Option<Self> {
        self.inner.checked_sub(rhs.inner).map(Quantity::new)
    }
}
