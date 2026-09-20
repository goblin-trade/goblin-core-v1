pub mod alias;
pub mod delta;
pub mod dim;
pub mod exp;

pub mod quantity_ops;
pub mod u32_quantity;
pub mod unsided;

pub use alias::*;
pub use delta::*;
pub use dim::*;
pub use exp::*;
pub use quantity_ops::*;
pub use u32_quantity::*;
pub use unsided::*;

mod impls;
#[cfg(test)]
mod tests;

use core::marker::PhantomData;

use deku::DekuRead;
#[cfg(feature = "encode")]
use deku::DekuWrite;
use goblin_macros::ConstDefault;

//
// Quantity type: value + Dim
//
#[derive(Default, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug, ConstDefault, DekuRead)]
#[cfg_attr(feature = "encode", derive(DekuWrite))]
pub struct Quantity<E, I>
where
    E: Exp,
    I: QuantityOps,
{
    pub inner: I,
    #[deku(skip)]
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

    /// Decode from the low bits of a raw bit field, delegating to the inner ops.
    #[inline]
    pub fn from_raw(raw: u64) -> Self {
        Self::new(I::from_raw(raw))
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
