use crate::settlement::global_delta::{ERC20SenderDeltas, EthDelta};

/// The global delta for msg.sender. It stores deltas for ETH and ERC20 tokens.
pub struct GlobalSenderDelta {
    /// Delta for ETH
    pub eth_delta: EthDelta,

    /// Deltas for ERC20 tokens. It has sub lists for hardcoded and custom tokens.
    pub erc20_deltas: ERC20SenderDeltas,
}

impl GlobalSenderDelta {
    pub const fn zero() -> Self {
        Self {
            eth_delta: EthDelta::zero(),
            erc20_deltas: ERC20SenderDeltas::zero(),
        }
    }
}
