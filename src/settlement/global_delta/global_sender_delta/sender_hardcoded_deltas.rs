use crate::{
    axis::token::token_marker::hardcoded_erc20::HARDCODED_TOKENS,
    settlement::global_delta::ERC20Delta,
};

/// Deltas of hardcoded tokens
pub type SenderHardcodedDeltas = [ERC20Delta; HARDCODED_TOKENS.len()];
