use crate::{
    goblin_error::GoblinError,
    hostio,
    input_processor::{DecodablePrimitive, DecodeCtx, HeaderFlags},
    quantities::{UnsidedAtoms, UnsidedDeltaAtoms},
    settlement::ConstZero,
};

/// The amount of ETH transfered in through msg_value and the amount due to be
/// transferred out. Used alongside EthDelta during settlement.
///
/// The unit of measurement is `UnsidedAtoms` obtained by downscaling RawAtoms
///
#[derive(Clone, Copy)]
pub struct ETHTransfers {
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

impl ETHTransfers {
    pub fn new(ctx: &DecodeCtx, flags: &HeaderFlags) -> Result<Self, GoblinError> {
        let msg_value = if flags.track_msg_value {
            let msg_value_raw = hostio::msg_value();
            UnsidedAtoms::try_from(msg_value_raw)?
        } else {
            UnsidedAtoms::ZEROED
        };

        let eth_out_due = if flags.withdraw_eth {
            UnsidedAtoms::decode_unchecked_no_advance(ctx)
        } else {
            UnsidedAtoms::ZEROED
        };

        Ok(Self {
            msg_value,
            eth_out_due,
        })
    }

    pub fn net_delta(&self) -> Result<UnsidedDeltaAtoms, GoblinError> {
        Ok(UnsidedDeltaAtoms::try_from(self.msg_value)?
            - UnsidedDeltaAtoms::try_from(self.eth_out_due)?)
    }
}
