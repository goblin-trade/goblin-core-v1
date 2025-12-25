use crate::{
    goblin_error::GoblinError,
    hostio::{self},
    input_processor::{ArgsBuffer, ArgsDecoder, Decodable, HeaderFlags, MarketCounts},
    quantities::{QuantityOps, UnsidedAtoms},
    types::NATIVE_TOKEN_DECIMALS,
};

/// Arguments read from calldata
pub struct GlobalHeader {
    /// Flags and counts. Tells whether optional values should be read.
    pub flags: HeaderFlags,

    /// Number of markets to decode, namespaced by type
    pub market_counts: MarketCounts,

    /// ETH atoms deposited via msg_value
    pub msg_value: UnsidedAtoms,

    /// ETH atoms to withdraw
    pub eth_out_due: UnsidedAtoms,
}

impl Decodable for GlobalHeader {
    fn decode(args: &ArgsBuffer, offset: &mut usize, len: usize) -> Result<Self, GoblinError> {
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

        Ok(Self {
            flags,
            market_counts,
            msg_value,
            eth_out_due,
        })
    }
}
