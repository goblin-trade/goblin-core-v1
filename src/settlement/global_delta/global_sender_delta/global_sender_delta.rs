use crate::{
    quantities::UnsidedAtoms,
    settlement::global_delta::{CustomTokenDeltas, EthDelta, HardcodedTokenDeltas},
};

/// The global delta for msg.sender.
///
/// It stores token wise deltas for 3 namepsaces
///
/// 1. ETH
/// 2. Hardcoded tokens
/// 3. Custom tokens
///
pub struct GlobalSenderDelta {
    /// Delta for ETH
    pub eth_delta: EthDelta,

    /// Deltas for hardcoded tokens
    pub hardcoded_token_deltas: HardcodedTokenDeltas,

    /// Deltas for custom tokens
    pub custom_token_deltas: CustomTokenDeltas,
}

impl GlobalSenderDelta {
    pub fn new(msg_value: UnsidedAtoms, eth_out_due: UnsidedAtoms) -> Self {
        Self {
            eth_delta: EthDelta::new(msg_value, eth_out_due),
            hardcoded_token_deltas: HardcodedTokenDeltas::default(),
            custom_token_deltas: CustomTokenDeltas::default(),
        }
    }
}
