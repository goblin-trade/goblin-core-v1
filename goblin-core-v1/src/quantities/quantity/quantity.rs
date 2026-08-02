use core::marker::PhantomData;

use crate::quantities::{Exp, QuantityOps};

//
// Quantity type: value + Dim
//
#[derive(Default, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
pub struct Quantity<E, I>
where
    E: Exp,
    I: QuantityOps,
{
    pub inner: I,
    _marker: PhantomData<E>,
}

impl<E, I> Quantity<E, I>
where
    E: Exp,
    I: QuantityOps,
{
    pub const fn new(value: I) -> Self {
        Self {
            inner: value,
            _marker: PhantomData,
        }
    }
}

impl<E, I> From<I> for Quantity<E, I>
where
    E: Exp,
    I: QuantityOps,
{
    fn from(value: I) -> Self {
        Self::new(value)
    }
}
