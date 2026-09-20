mod impl_deku_decode;
mod impl_variable_decode;

use crate::{axis::token::TokenDataTriple, types::Address};

/// Zero copy values read from calldata, decoded after the header
pub struct HeaderRefs<'a> {
    /// Optional custom recipient
    pub custom_recipient: Option<&'a Address>,

    pub token_data_triple: TokenDataTriple<'a>,
}
