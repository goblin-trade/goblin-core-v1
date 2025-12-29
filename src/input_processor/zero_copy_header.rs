use crate::{
    input_processor::{DecodeCtx, GlobalHeader},
    token::CustomToken,
    types::Address,
};

/// Zero copy deserialized header fields
pub struct ZeroCopyHeader<'a> {
    /// Optional custom recipient
    pub recipient: Option<&'a Address>,

    /// Addresses of custom erc20 tokens to use
    pub custom_erc20_list: &'a [CustomToken],
}

impl<'a> ZeroCopyHeader<'a> {
    pub fn new(ctx: &'a DecodeCtx, global_header: &GlobalHeader) -> Self {
        let recipient = if global_header.flags.recipient_provided {
            Some(ctx.decode_ref_unchecked::<Address>())
        } else {
            None
        };

        let custom_erc20_list =
            ctx.decode_slice_unchecked::<CustomToken>(global_header.flags.custom_erc20_count);

        Self {
            recipient,
            custom_erc20_list,
        }
    }
}
