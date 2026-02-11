use crate::{
    axis::token::{token_marker::hardcoded_erc20::HARDCODED_TOKENS, Token},
    settlement::global_delta::{
        ERC20Delta, EthDelta, SenderCustomDeltas, SenderHardcodedDeltas, MAX_CUSTOM_DELTAS,
    },
    types::Triple,
};

/// The global delta for msg.sender. Stores deltas of ETH and ERC20 tokens.
///
/// TODO remove deposit?
/// Deposit is handled elsewhere. Can we use UnsidedSenderDelta directly?
pub type GlobalSenderDelta = Triple<EthDelta, SenderHardcodedDeltas, SenderCustomDeltas, Token>;

impl GlobalSenderDelta {
    pub const fn zero() -> Self {
        Self::new(
            EthDelta::zero(),
            [ERC20Delta::zero(); HARDCODED_TOKENS.len()],
            [ERC20Delta::zero(); MAX_CUSTOM_DELTAS],
        )
    }
}
