use crate::{goblin_error::GoblinError, quantities::UnsidedAtoms};

/// The amount of ETH transfered in through msg_value and the amount due to be
/// transferred out. Used alongside EthDelta during settlement.
///
/// The unit of measurement is `UnsidedAtoms` obtained by downscaling RawAtoms
///
#[derive(Clone, Copy)]
pub struct ETHTransfers {
    /// ETH atoms deposited via msg_value
    pub msg_value: UnsidedAtoms<u64>,

    /// Amount of ETH atoms pending withdrawal, as read from global namespace header
    ///
    /// The actual amount withdrawn is MIN(available, widthdrawal_due)
    /// This allows us to withdraw max available amount by passing u64::MAX
    ///
    /// The amount is transferred out internally (store credit) or externally (transfer call).
    pub eth_out_due: UnsidedAtoms<u64>,
}

impl ETHTransfers {
    pub fn net_delta(&self) -> Result<UnsidedAtoms<i64>, GoblinError> {
        Ok(UnsidedAtoms::<i64>::try_from(self.msg_value)?
            - UnsidedAtoms::<i64>::try_from(self.eth_out_due)?)
    }
}
