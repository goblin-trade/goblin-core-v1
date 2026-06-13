use crate::{
    axis::token::{token_marker::hardcoded_erc20::HARDCODED_TOKENS, Token},
    settlement::{
        global_delta::{
            ERC20Delta, EthDelta, SenderCustomDeltas, SenderHardcodedDeltas, MAX_CUSTOM_DELTAS,
        },
        ConstZero,
    },
    types::Triple,
};

/// The global delta for msg.sender. Stores deltas of ETH and ERC20 tokens.
pub type GlobalSenderDelta = Triple<EthDelta, SenderHardcodedDeltas, SenderCustomDeltas, Token>;

impl ConstZero for GlobalSenderDelta {
    const ZEROED: Self = Self::new(
        EthDelta::ZEROED,
        [ERC20Delta::ZEROED; HARDCODED_TOKENS.len()],
        [ERC20Delta::ZEROED; MAX_CUSTOM_DELTAS],
    );
}
