use crate::settlement::global_delta::{EthDelta, TokenSenderDeltas};

/// The global delta for msg.sender. It stores deltas for ETH and ERC20 tokens.
pub struct GlobalSenderDelta {
    /// Delta for ETH
    pub eth_delta: EthDelta,

    /// Deltas for ERC20 tokens. It has sub lists for hardcoded and custom tokens.
    pub token_deltas: TokenSenderDeltas,
}

impl GlobalSenderDelta {
    pub const fn new() -> Self {
        Self {
            eth_delta: EthDelta::new(),
            token_deltas: TokenSenderDeltas::new(),
        }
    }
}
