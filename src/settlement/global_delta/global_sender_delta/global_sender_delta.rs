use crate::{
    settlement::global_delta::{ERC20SenderDeltas, EthDelta},
    types::TokenPair,
};

/// The global delta for msg.sender. Stores deltas of ETH and ERC20 tokens.
pub type GlobalSenderDelta = TokenPair<EthDelta, ERC20SenderDeltas>;

impl GlobalSenderDelta {
    pub const fn zero() -> Self {
        Self::new(EthDelta::zero(), ERC20SenderDeltas::zero())
    }
}
