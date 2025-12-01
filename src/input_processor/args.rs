use crate::{
    goblin_error::GoblinError,
    hostio::{self, HostioContext},
    input_processor::{ArgsDecoder, Header},
    quantities::{QuantityOps, UnsidedAtoms},
    tokens::CustomToken,
    types::{Address, NATIVE_TOKEN_DECIMALS},
};

/// Arguments read from calldata
pub struct Args<'a> {
    /// Flags and counts. Tells whether optional values should be read.
    pub header: Header,

    /// ETH atoms deposited via msg_value
    pub msg_value: UnsidedAtoms,

    /// ETH atoms to withdraw
    pub eth_out_due: UnsidedAtoms,

    /// Optional custom recipient
    pub recipient: Option<&'a Address>,

    /// Addresses of custom erc20 tokens to use
    pub custom_erc20_list: &'a [CustomToken],
}

impl<'a> Args<'a> {
    pub fn new(
        ctx: &'a HostioContext,
        offset: &mut usize,
        len: usize,
    ) -> Result<Self, GoblinError> {
        let header = Header::init(&ctx.args, len)?;
        *offset = Header::HEADER_BYTE_SIZE;

        let msg_value = if header.track_msg_value {
            let msg_value_raw = hostio::msg_value();
            UnsidedAtoms::from_raw_atoms(msg_value_raw.as_ref(), NATIVE_TOKEN_DECIMALS)?
        } else {
            UnsidedAtoms::ZERO
        };

        let eth_out_due = if header.withdraw_eth {
            *ctx.args.decode_ref_unchecked::<UnsidedAtoms>(offset)
        } else {
            UnsidedAtoms::ZERO
        };

        let recipient = if header.recipient_provided {
            Some(ctx.args.decode_ref_unchecked::<Address>(offset))
        } else {
            None
        };

        let custom_erc20_list = ctx
            .args
            .decode_slice_unchecked::<CustomToken>(offset, header.custom_erc20_count);

        Ok(Args {
            header,
            recipient,
            msg_value,
            eth_out_due,
            custom_erc20_list,
        })
    }
}
