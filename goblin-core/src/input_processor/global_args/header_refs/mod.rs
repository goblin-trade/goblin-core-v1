mod header_refs_ctx;

pub use header_refs_ctx::HeaderRefsCtx;

use deku::DekuRead;
#[cfg(feature = "encode")]
use deku::DekuWrite;

use crate::{axis::token::TokenDataTriple, types::Address};

/// Zero copy values read from calldata, decoded after the header
///
/// Reading borrows zero-copy out of the calldata slice carried by
/// [`HeaderRefsCtx`], which is why both fields use a custom `#[deku(reader)]`.
/// The same ctx drives writing; only its flags are consulted there.
#[derive(DekuRead)]
#[cfg_attr(feature = "encode", derive(DekuWrite))]
#[deku(ctx = "ctx: HeaderRefsCtx<'a>")]
pub struct HeaderRefs<'a> {
    /// Optional custom recipient
    #[deku(reader = "ctx.read_recipient(deku::reader)")]
    #[cfg_attr(
        feature = "encode",
        deku(writer = "ctx.write_recipient(deku::writer, &self.custom_recipient)")
    )]
    pub custom_recipient: Option<&'a Address>,

    /// ETH, hardcoded ERC20 and custom ERC20 token lists. Only the custom list is
    /// carried in calldata, so it is the only one serialized.
    #[deku(reader = "ctx.read_token_data_triple(deku::reader)")]
    #[cfg_attr(
        feature = "encode",
        deku(writer = "ctx.write_token_data_triple(deku::writer, &self.token_data_triple)")
    )]
    pub token_data_triple: TokenDataTriple<'a>,
}
