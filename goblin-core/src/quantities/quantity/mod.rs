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

use crate::codec::{CodecResult, GoblinRead, Reader};
#[cfg(feature = "encode")]
use crate::codec::{GoblinWrite, Writer};
use goblin_macros::ConstDefault;

//
// Quantity type: value + Dim
//
#[derive(Default, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug, ConstDefault)]
pub struct Quantity<E, I>
where
    E: Exp,
    I: QuantityOps,
{
    pub inner: I,
    _marker: PhantomData<E>,
}

impl<'de, E, I> GoblinRead<'de, ()> for Quantity<E, I>
where
    E: Exp,
    I: QuantityOps,
{
    fn from_reader_with_ctx(reader: &mut Reader<'de>, (): ()) -> CodecResult<Self> {
        let inner = I::from_reader_with_ctx(reader, ())?;
        Ok(Self {
            inner,
            _marker: PhantomData,
        })
    }
}

#[cfg(feature = "encode")]
impl<E, I> GoblinWrite<()> for Quantity<E, I>
where
    E: Exp,
    I: QuantityOps,
{
    fn to_writer(&self, writer: &mut Writer<'_>, (): ()) -> CodecResult<()> {
        self.inner.to_writer(writer, ())
    }
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
