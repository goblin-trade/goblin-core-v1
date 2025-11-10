///! Arguments read from calldata.
///!
///! * We use a custom deserialization format that begins with a header. The header tells how bytes should be interpreted.
///! For example args can contain an optional fields like `recipient` address and `eth_withdrawal_due`,
///! along with slices of variable length.
///!
///! *
use crate::{
    goblin_error::GoblinError,
    hostio,
    input_processor::{ArgsBuffer, ArgsDecoder, Header},
    quantities::{QuantityOps, UnsidedAtoms},
    tokens::CustomToken,
    types::{Address, NATIVE_TOKEN_DECIMALS},
};

pub struct Args<'a> {
    /// Flags and counts
    pub header: Header,

    /// ETH atoms deposited via msg_value
    pub msg_value: UnsidedAtoms,

    /// ETH atoms to withdraw
    pub eth_out_due: UnsidedAtoms,

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

        let msg_value = if header.track_msg_value {
            let msg_value_raw = hostio::msg_value();
            UnsidedAtoms::from_raw_atoms(msg_value_raw.as_ref(), NATIVE_TOKEN_DECIMALS)?
        } else {
            UnsidedAtoms::ZERO
        };

        let eth_out = match header.withdraw_eth {
            true => *payload.decode_ref_unchecked::<UnsidedAtoms>(&mut offset),
            false => UnsidedAtoms::ZERO,
        };

        let recipient = header
            .recipient_provided
            .then(|| payload.decode_ref_unchecked::<Address>(&mut offset));

        let custom_erc20_list =
            payload.decode_slice_unchecked::<CustomToken>(&mut offset, header.custom_erc20_count);

        Ok(Args {
            header,
            recipient,
            msg_value,
            eth_out_due: eth_out,
            custom_erc20_list,
            offset,
        })
    }
}
