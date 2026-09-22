mod impl_deku_decode;

use crate::{axis::token::TokenDataTriple, input_processor::HeaderFlags, types::Address};

/// Zero copy values read from calldata, decoded after the header
pub struct HeaderRefs<'a> {
    /// Optional custom recipient
    pub custom_recipient: Option<&'a Address>,

    pub token_data_triple: TokenDataTriple<'a>,
}

/// Deku decoding context for [`HeaderRefs`].
///
/// Zero-copy borrowing needs the reader's backing `&[u8]`, but a
/// [`DekuReader`] impl is generic over the reader `R` and cannot recover that
/// slice. It is therefore threaded through the context, alongside the header
/// flags.
///
/// [`DekuReader`]: deku::DekuReader
pub struct HeaderRefsCtx<'a> {
    pub flags: HeaderFlags,
    pub source: &'a [u8],
}
