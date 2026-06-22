use crate::{
    input_processor::{DecodablePrimitive, DecodeCtx},
    quantities::{Exp, Quantity, QuantityOps},
    settlement::{CheckedOps, ConstZero},
};

// Implementation for constants, addition and subtraction.
// These will be used in the leg namespace.
//
// Multiplication and division operations are asymmetric and happen
// in the side namespace.
impl<E, I> QuantityOps for Quantity<E, I>
where
    E: Exp,
    I: QuantityOps,
{
    const MIN: Self = Self::new(I::MIN);
    const MAX: Self = Self::new(I::MAX);
    const ONE: Self = Self::new(I::ONE);
}

impl<E, I> ConstZero for Quantity<E, I>
where
    E: Exp,
    I: QuantityOps,
{
    const ZEROED: Self = Self::new(I::ZEROED);
}

impl<E, I> CheckedOps for Quantity<E, I>
where
    E: Exp,
    I: QuantityOps,
{
    fn checked_add(self, rhs: Self) -> Option<Self> {
        self.inner.checked_add(rhs.inner).map(Quantity::new)
    }

    fn checked_sub(self, rhs: Self) -> Option<Self> {
        self.inner.checked_sub(rhs.inner).map(Quantity::new)
    }
}

impl<E, I> DecodablePrimitive for Quantity<E, I>
where
    E: Exp,
    I: QuantityOps,
{
    fn decode_unchecked_no_advance(ctx: &DecodeCtx) -> Self {
        Self::new(I::decode_unchecked_no_advance(ctx))
    }
}
