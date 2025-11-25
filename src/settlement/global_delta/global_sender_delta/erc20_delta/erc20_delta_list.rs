use crate::{
    settlement::global_delta::LazyERC20Delta,
    tokens::{CustomToken, ERC20TokenTrait, HardcodedToken, TokenIndex, HARDCODED_TOKENS},
};

pub const MAX_CUSTOM_DELTAS: usize = 8;

/// The list of ERC20 deltas. These deltas are stored in a fixed size list, indexed by
/// the token index.
pub trait ERC20DeltaList<T: ERC20TokenTrait> {
    /// Get a mutable reference to LazyERC20Delta for the given token index
    ///
    /// The ERC20Delta struct is wrapped in MaybeUninit and has an `init` flag.
    /// This format saves compute by avoiding zero-filling ERC20Delta. If `init` is false,
    /// we overwrite the ERC20Delta and set `init` to true.
    fn get_delta_mut(&mut self, token_index: TokenIndex<T>) -> &mut LazyERC20Delta;
}

/// List of hardcoded token deltas. The tokens and their indices are hardcoded in the contract.
pub type HardcodedTokenDeltas = [LazyERC20Delta; HARDCODED_TOKENS.len()];

/// List of custom token deltas. Since custom tokens are passed at runtime, the mapping between
/// token index and token address can vary across contract calls.
///
/// # Safety
///
/// TokenIndex<CustomToken>::new() ensures that index is within MAX_CUSTOM_DELTAS bounds.
/// Therefore lookups are safe.
pub type CustomTokenDeltas = [LazyERC20Delta; MAX_CUSTOM_DELTAS];

impl ERC20DeltaList<HardcodedToken> for HardcodedTokenDeltas {
    fn get_delta_mut(&mut self, token_index: TokenIndex<HardcodedToken>) -> &mut LazyERC20Delta {
        &mut self[token_index.inner as usize]
    }
}

impl ERC20DeltaList<CustomToken> for CustomTokenDeltas {
    fn get_delta_mut(&mut self, token_index: TokenIndex<CustomToken>) -> &mut LazyERC20Delta {
        &mut self[token_index.inner as usize]
    }
}
