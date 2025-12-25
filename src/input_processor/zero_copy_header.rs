use crate::{
    input_processor::{ArgsBuffer, ArgsDecoder, GlobalHeader},
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
    pub fn new(global_header: &GlobalHeader, args: &'a ArgsBuffer, offset: &mut usize) -> Self {
        let recipient = if global_header.flags.recipient_provided {
            Some(args.decode_ref_unchecked::<Address>(offset))
        } else {
            None
        };

        let custom_erc20_list = args
            .decode_slice_unchecked::<CustomToken>(offset, global_header.flags.custom_erc20_count);

        Self {
            recipient,
            custom_erc20_list,
        }
    }
}
