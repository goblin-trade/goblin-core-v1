use crate::{
    settlement::global_delta::{ERC20Delta, MAX_CUSTOM_DELTAS},
    token::HARDCODED_TOKENS,
};

pub struct TokenSenderDeltas {
    /// Deltas for hardcoded tokens
    pub hardcoded_token_deltas: HardcodedTokenDeltas,

    /// Deltas for custom tokens
    pub custom_token_deltas: CustomTokenDeltas,
}

impl TokenSenderDeltas {
    pub const fn new() -> Self {
        Self {
            hardcoded_token_deltas: [ERC20Delta::new(); HARDCODED_TOKENS.len()],
            custom_token_deltas: [ERC20Delta::new(); MAX_CUSTOM_DELTAS],
        }
    }
}

/// List of hardcoded token deltas. The tokens and their indices are hardcoded in the contract.
pub type HardcodedTokenDeltas = [ERC20Delta; HARDCODED_TOKENS.len()];

/// List of custom token deltas. Since custom tokens are passed at runtime, the mapping between
/// token index and token address can vary across contract calls.
///
/// # Safety
///
/// TokenIndex<CustomToken>::new() ensures that index is within MAX_CUSTOM_DELTAS bounds.
/// Therefore lookups are safe.
pub type CustomTokenDeltas = [ERC20Delta; MAX_CUSTOM_DELTAS];
