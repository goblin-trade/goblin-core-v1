use crate::{
    settlement::global_delta::{ERC20Delta, Lazy, MAX_CUSTOM_DELTAS},
    tokens::HARDCODED_TOKENS,
};

#[derive(Default)]
pub struct TokenDeltas {
    /// Deltas for hardcoded tokens
    pub hardcoded_token_deltas: HardcodedTokenDeltas,

    /// Deltas for custom tokens
    pub custom_token_deltas: CustomTokenDeltas,
}

/// List of hardcoded token deltas. The tokens and their indices are hardcoded in the contract.
pub type HardcodedTokenDeltas = [Lazy<ERC20Delta>; HARDCODED_TOKENS.len()];

/// List of custom token deltas. Since custom tokens are passed at runtime, the mapping between
/// token index and token address can vary across contract calls.
///
/// # Safety
///
/// TokenIndex<CustomToken>::new() ensures that index is within MAX_CUSTOM_DELTAS bounds.
/// Therefore lookups are safe.
pub type CustomTokenDeltas = [Lazy<ERC20Delta>; MAX_CUSTOM_DELTAS];
