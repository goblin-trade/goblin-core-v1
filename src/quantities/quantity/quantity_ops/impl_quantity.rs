use crate::{
    quantities::{Exp, Quantity, QuantityOps},
    settlement::{CheckedOps, ConstZero},
};

// Implementation for constants, addition and subtraction.
// These will be used in the leg namespace.
//
// Multiplication and division operations are asymmetric and happen
// in the side namespace.
impl<E> QuantityOps for Quantity<E>
where
    E: Exp + Copy + PartialEq + Default + PartialOrd + Ord,
{
    const MIN: Self = Self::new(u64::MIN);
    const MAX: Self = Self::new(u64::MAX);
    const ONE: Self = Self::new(1);
}

impl<E> ConstZero for Quantity<E>
where
    E: Exp + Copy + PartialEq + Default + PartialOrd + Ord,
{
    const ZEROED: Self = Self::new(0);
}

impl<E> CheckedOps for Quantity<E>
where
    E: Exp + Copy + PartialEq + Default + PartialOrd + Ord,
{
    fn checked_add(self, rhs: Self) -> Option<Self> {
        self.inner.checked_add(rhs.inner).map(Quantity::new)
    }

    fn checked_sub(self, rhs: Self) -> Option<Self> {
        self.inner.checked_sub(rhs.inner).map(Quantity::new)
    }
}
