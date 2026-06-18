use crate::{
    axis::token::{token_reader::hardcoded_erc20::HARDCODED_TOKENS, HardcodedERC20},
    settlement::{global_delta::SenderTokenStore, ConstZero},
};

/// Deltas of hardcoded tokens
pub type SenderHardcodedDeltas = [SenderTokenStore<HardcodedERC20>; HARDCODED_TOKENS.len()];

impl ConstZero for SenderHardcodedDeltas {
    const ZEROED: Self = [SenderTokenStore::ZEROED; HARDCODED_TOKENS.len()];
}
