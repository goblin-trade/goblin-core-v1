mod custom_erc20;
mod eth;
mod hardcoded_erc20;

use crate::{goblin_error::GoblinError, quantities::UnsidedDeltaAtoms};

/// Tokens transferred at the top level through calldata
///
/// * ETH is deposited via msg.value. ETH withdraw amount is namespaced at calldata level
///   not market namespace level.
///
/// * ERC20 tokens deltas are read at the market level. They are stubs in the calldata level.
pub trait TokenMsgTransfer {
    fn net_delta(&self) -> Result<UnsidedDeltaAtoms, GoblinError>;

    fn deposit_due(&self) -> Result<UnsidedDeltaAtoms, GoblinError>;
}
