mod alias;
pub mod delta;
pub mod dim;
pub mod exp;

pub mod quantity_ops;
pub mod unsided;

pub use alias::*;
pub use delta::*;
pub use dim::*;
pub use exp::*;
pub use quantity_ops::*;
pub use unsided::*;

mod impls;
#[cfg(test)]
mod tests;

use core::marker::PhantomData;

use goblin_macros::{ConstDefault, FixedDecode};

//
// Quantity type: value + Dim
//
#[derive(
    Default, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug, FixedDecode, ConstDefault,
)]
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
