///! Arguments read from calldata.
///!
///! * We use a custom deserialization format that begins with a header. The header tells how bytes should be interpreted.
///! For example args can contain an optional fields like `recipient` address and `eth_withdrawal_due`,
///! along with slices of variable length.
///!
///! *
use crate::{
    goblin_error::GoblinError,
    input_processor::{ArgsBuffer, ArgsDecoder, Header},
    tokens::CustomToken,
    types::Address,
};

pub struct Args<'a> {
    /// Flags and counts
    pub header: Header,

    /// Optional custom recipient
    pub recipient: Option<&'a Address>,

    /// Addresses of custom erc20 tokens to use
    pub custom_erc20_list: &'a [CustomToken],

    pub offset: usize,
}

impl<'a> Args<'a> {
    pub fn new(payload: &'a ArgsBuffer, len: usize) -> Result<Self, GoblinError> {
        let header = Header::init(payload, len)?;
        let mut offset = Header::HEADER_BYTE_SIZE;

        let provided_recipient = header
            .recipient_provided
            .then(|| payload.decode_ref_unchecked::<Address>(&mut offset));

        let custom_erc20_list =
            payload.decode_slice_unchecked::<CustomToken>(&mut offset, header.custom_erc20_count);

        Ok(Args {
            header,
            recipient: provided_recipient,
            custom_erc20_list,
            offset,
        })
    }
}
