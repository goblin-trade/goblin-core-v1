use crate::{
    settlement::global_delta::{ERC20Delta, MAX_CUSTOM_DELTAS},
    token::HARDCODED_TOKENS,
    types::ERC20Pair,
};

/// Deltas of ERC20 tokens for the sender, hardcoded and custom
pub type ERC20SenderDeltas = ERC20Pair<SenderHardcodedDeltas, SenderCustomDeltas>;

impl ERC20SenderDeltas {
    pub const fn zero() -> Self {
        Self::new(
            [ERC20Delta::zero(); HARDCODED_TOKENS.len()],
            [ERC20Delta::zero(); MAX_CUSTOM_DELTAS],
        )
    }
}

/// Deltas of hardcoded tokens
pub type SenderHardcodedDeltas = [ERC20Delta; HARDCODED_TOKENS.len()];

/// Deltas of custom tokens.
///
/// # Safety
///
/// TokenIndex<CustomToken>::new() ensures that index is within MAX_CUSTOM_DELTAS bounds.
/// Therefore lookups are safe.
pub type SenderCustomDeltas = [ERC20Delta; MAX_CUSTOM_DELTAS];
