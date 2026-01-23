use crate::{
    settlement::global_delta::{
        ERC20Delta, EthDelta, SenderCustomDeltas, SenderHardcodedDeltas, MAX_CUSTOM_DELTAS,
    },
    token::HARDCODED_TOKENS,
    types::TokenTriple,
};

/// The global delta for msg.sender. Stores deltas of ETH and ERC20 tokens.
pub type GlobalSenderDelta = TokenTriple<EthDelta, SenderHardcodedDeltas, SenderCustomDeltas>;

impl GlobalSenderDelta {
    pub const fn zero() -> Self {
        Self::new(
            EthDelta::zero(),
            [ERC20Delta::zero(); HARDCODED_TOKENS.len()],
            [ERC20Delta::zero(); MAX_CUSTOM_DELTAS],
        )
    }
}
