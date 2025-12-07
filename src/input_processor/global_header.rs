use crate::{
    goblin_error::GoblinError,
    hostio::{self},
    input_processor::{ArgsBuffer, ArgsDecoder, Decodable, HeaderFlags, MarketCounts},
    quantities::{QuantityOps, UnsidedAtoms},
    token::CustomToken,
    types::{Address, NATIVE_TOKEN_DECIMALS},
};

/// Arguments read from calldata
pub struct GlobalHeader<'a> {
    /// Flags and counts. Tells whether optional values should be read.
    pub flags: HeaderFlags,

    /// Number of markets to decode, namespaced by type
    pub market_counts: MarketCounts,

    /// ETH atoms deposited via msg_value
    pub msg_value: UnsidedAtoms,

    /// ETH atoms to withdraw
    pub eth_out_due: UnsidedAtoms,

    /// Optional custom recipient
    pub recipient: Option<&'a Address>,

    /// Addresses of custom erc20 tokens to use
    pub custom_erc20_list: &'a [CustomToken],
}

impl<'a> GlobalHeader<'a> {
    /// Decode the global header from args
    ///
    /// The API is similar to Decodable trait includes the 'a lifetime that Decodable lacks
    pub fn new(args: &'a ArgsBuffer, offset: &mut usize, len: usize) -> Result<Self, GoblinError> {
        let flags = HeaderFlags::decode(args, offset, len)?;
        let market_counts = MarketCounts::decode(args, offset, len)?;

        let msg_value = if flags.track_msg_value {
            let msg_value_raw = hostio::msg_value();
            UnsidedAtoms::from_raw_atoms(msg_value_raw.as_ref(), NATIVE_TOKEN_DECIMALS)?
        } else {
            UnsidedAtoms::ZERO
        };

        let eth_out_due = if flags.withdraw_eth {
            *args.decode_ref_unchecked::<UnsidedAtoms>(offset)
        } else {
            UnsidedAtoms::ZERO
        };

        let recipient = if flags.recipient_provided {
            Some(args.decode_ref_unchecked::<Address>(offset))
        } else {
            None
        };

        let custom_erc20_list =
            args.decode_slice_unchecked::<CustomToken>(offset, flags.custom_erc20_count);

        Ok(GlobalHeader {
            flags,
            market_counts,
            recipient,
            msg_value,
            eth_out_due,
            custom_erc20_list,
        })
    }
}
