use crate::codec::{CodecError, CodecResult, GoblinRead, Reader};
#[cfg(feature = "encode")]
use crate::codec::{GoblinWrite, Writer};
use goblin_macros::ConstDefault;

#[derive(Clone, Copy, PartialEq, PartialOrd, ConstDefault)]
pub struct CustomERC20Index {
    pub inner: usize,
}

impl<'de> GoblinRead<'de, ()> for CustomERC20Index {
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
impl GoblinWrite<()> for CustomERC20Index {
    fn to_writer(&self, writer: &mut Writer<'_>, (): ()) -> CodecResult<()> {
        writer.write_u8(self.inner as u8)
    }
}

impl CustomERC20Index {
    /// 0b111 = 7 as it is decoded from 3 bits.
    pub const MAX_INNER: usize = 7;
}

impl From<usize> for CustomERC20Index {
    fn from(value: usize) -> Self {
        Self { inner: value }
    }
}
