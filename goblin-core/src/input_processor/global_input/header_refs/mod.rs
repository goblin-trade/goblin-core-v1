mod header_refs_ctx;

pub use header_refs_ctx::HeaderRefsCtx;

use crate::codec::{CodecResult, GoblinRead, Reader};
#[cfg(feature = "encode")]
use crate::codec::{GoblinWrite, Writer};

use crate::{axis::token::TokenDataTriple, types::Address};

/// Zero copy values read from calldata, decoded after the header
///
/// Reading borrows zero-copy out of the reader's backing slice, which the
/// context carries the flags for.
pub struct HeaderRefs<'a> {
    /// Optional custom recipient
    pub custom_recipient: Option<&'a Address>,

    /// ETH, hardcoded ERC20 and custom ERC20 token lists. Only the custom list is
    /// carried in calldata, so it is the only one serialized.
    pub token_data_triple: TokenDataTriple<'a>,
}

impl<'de> GoblinRead<'de, HeaderRefsCtx> for HeaderRefs<'de> {
    fn from_reader_with_ctx(reader: &mut Reader<'de>, ctx: HeaderRefsCtx) -> CodecResult<Self> {
        let custom_recipient = ctx.read_recipient(reader)?;
        let token_data_triple = ctx.read_token_data_triple(reader)?;

        Ok(Self {
            custom_recipient,
            token_data_triple,
        })
    }
}

#[cfg(feature = "encode")]
impl<'de> GoblinWrite<HeaderRefsCtx> for HeaderRefs<'de> {
    fn to_writer(&self, writer: &mut Writer<'_>, ctx: HeaderRefsCtx) -> CodecResult<()> {
        ctx.write_recipient(writer, &self.custom_recipient)?;
        ctx.write_token_data_triple(writer, &self.token_data_triple)
    }
}
