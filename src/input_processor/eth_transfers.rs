use crate::{
    goblin_error::GoblinError,
    hostio,
    input_processor::{DecodeCtx, DecodablePrimitive, HeaderFlags},
    quantities::{QuantityOps, UnsidedAtoms},
    types::NATIVE_TOKEN_DECIMALS,
};

/// The amount of ETH transfered in through msg_value and the amount due to be
/// transferred out. Used alongside EthDelta during settlement.
///
/// The unit of measurement is `UnsidedAtoms` obtained by downscaling RawAtoms
///
#[derive(Clone, Copy)]
pub struct EthTransfers {
    /// ETH atoms deposited via msg_value
    pub msg_value: UnsidedAtoms,

    /// Amount of ETH atoms pending withdrawal, as read from global namespace header
    ///
    /// The actual amount withdrawn is MIN(available, widthdrawal_due)
    /// This allows us to withdraw max available amount by passing u64::MAX
    ///
    /// The amount is transferred out internally (store credit) or externally (transfer call).
    pub eth_out_due: UnsidedAtoms,
}

impl EthTransfers {
    pub fn new(ctx: &DecodeCtx, flags: &HeaderFlags) -> Result<Self, GoblinError> {
        let msg_value = if flags.track_msg_value {
            let msg_value_raw = hostio::msg_value();
            UnsidedAtoms::from_raw_atoms(msg_value_raw.as_ref(), NATIVE_TOKEN_DECIMALS)?
        } else {
            UnsidedAtoms::ZERO
        };

        let eth_out_due = if flags.withdraw_eth {
            UnsidedAtoms::decode_unchecked_no_advance(ctx)
        } else {
            UnsidedAtoms::ZERO
        };

        Ok(Self {
            msg_value,
            eth_out_due,
        })
    }
}
