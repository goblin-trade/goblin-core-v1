use crate::{
    quantities::UnsidedAtoms,
    settlement::global_delta::{EthDelta, TokenDeltas},
};

/// The global delta for msg.sender. It stores deltas for ETH and ERC20 tokens.
pub struct GlobalSenderDelta {
    /// Delta for ETH
    pub eth_delta: EthDelta,

    /// Deltas for ERC20 tokens. It has sub lists for hardcoded and custom tokens.
    pub token_deltas: TokenDeltas,
}

impl GlobalSenderDelta {
    pub fn new(msg_value: UnsidedAtoms, eth_out_due: UnsidedAtoms) -> Self {
        Self {
            eth_delta: EthDelta::new(msg_value, eth_out_due),
            token_deltas: TokenDeltas::default(),
        }
    }
}
