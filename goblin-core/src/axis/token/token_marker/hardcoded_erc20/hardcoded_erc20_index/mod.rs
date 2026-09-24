use crate::codec::{CodecError, CodecResult, GoblinRead, Reader};
#[cfg(feature = "encode")]
use crate::codec::{GoblinWrite, Writer};
use goblin_macros::ConstDefault;

mod impl_index;

use crate::axis::token::token_list::HARDCODED_ERC20_COUNT;

#[derive(Clone, Copy, PartialEq, PartialOrd, ConstDefault)]
pub struct HardcodedERC20Index {
    pub inner: usize,
}

impl<'de> GoblinRead<'de, ()> for HardcodedERC20Index {
    fn from_reader_with_ctx(reader: &mut Reader<'de>, (): ()) -> CodecResult<Self> {
        let inner = reader.read_u8()? as usize;
        if inner <= Self::MAX_INNER {
            Ok(Self { inner })
        } else {
            Err(CodecError::InvalidValue)
        }
    }
}

#[cfg(feature = "encode")]
impl GoblinWrite<()> for HardcodedERC20Index {
    fn to_writer(&self, writer: &mut Writer<'_>, (): ()) -> CodecResult<()> {
        writer.write_u8(self.inner as u8)
    }
}

impl HardcodedERC20Index {
    pub const MAX_INNER: usize = HARDCODED_ERC20_COUNT - 1;

    pub const fn new(inner: usize) -> Self {
        Self { inner }
    }
}

impl From<usize> for HardcodedERC20Index {
    fn from(value: usize) -> Self {
        Self::new(value)
    }
}
