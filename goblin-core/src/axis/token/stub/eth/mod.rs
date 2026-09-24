use goblin_macros::ConstDefault;

use crate::codec::{CodecResult, GoblinRead, Reader};
#[cfg(feature = "encode")]
use crate::codec::{GoblinWrite, Writer};

mod impl_checked_ops;
mod impl_from;
mod impl_index;
mod impl_into_iterator;

/// Stub type for ETH token index, address and deposit
///
/// Use an explicit stub type instead of `()` for clarity
#[derive(Default, Clone, Copy, PartialEq, ConstDefault)]
pub struct ETHStub;

impl<'de> GoblinRead<'de, ()> for ETHStub {
    #[inline]
    fn from_reader_with_ctx(_reader: &mut Reader<'de>, (): ()) -> CodecResult<Self> {
        Ok(ETHStub)
    }
}

#[cfg(feature = "encode")]
impl GoblinWrite<()> for ETHStub {
    #[inline]
    fn to_writer(&self, _writer: &mut Writer<'_>, (): ()) -> CodecResult<()> {
        Ok(())
    }
}
