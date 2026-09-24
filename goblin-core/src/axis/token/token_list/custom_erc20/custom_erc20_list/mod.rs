mod custom_erc20_list_ctx;
mod impl_index;
mod impl_into_iterator;

pub use custom_erc20_list_ctx::CustomERC20ListCtx;

use crate::axis::token::{CustomERC20, token_marker::TokenData};
use crate::codec::{CodecResult, GoblinRead, Reader};
#[cfg(feature = "encode")]
use crate::codec::{GoblinWrite, Writer};

/// Zero copy custom ERC20 token list read from calldata.
///
/// Reading borrows zero-copy out of the reader's backing slice; only the token
/// count travels in [`CustomERC20ListCtx`].
#[derive(Clone, Copy, Default)]
pub struct CustomERC20List<'a> {
    /// Concatenated token addresses, no length prefix.
    pub inner: &'a [TokenData<CustomERC20>],
}

impl<'de> GoblinRead<'de, CustomERC20ListCtx> for CustomERC20List<'de> {
    fn from_reader_with_ctx(
        reader: &mut Reader<'de>,
        ctx: CustomERC20ListCtx,
    ) -> CodecResult<Self> {
        Ok(Self {
            inner: ctx.read_tokens(reader)?,
        })
    }
}

#[cfg(feature = "encode")]
impl<'de> GoblinWrite<CustomERC20ListCtx> for CustomERC20List<'de> {
    fn to_writer(&self, writer: &mut Writer<'_>, ctx: CustomERC20ListCtx) -> CodecResult<()> {
        ctx.write_tokens(writer, self.inner)
    }
}
