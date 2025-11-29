use crate::{
    settlement::global_delta::{CustomTokenDeltas, ERC20Delta, HardcodedTokenDeltas, Lazy},
    tokens::{CustomToken, ERC20Token, HardcodedToken, TokenIndex},
};

pub const MAX_CUSTOM_DELTAS: usize = 8;

/// The list of ERC20 deltas. These deltas are stored in a fixed size list, indexed by
/// the token index.
///
/// This function just maps index to delta. It doesn't deal with the wrapper struct.
pub trait ERC20DeltaList<T: ERC20Token> {
    /// Get a mutable reference to Lazy<ERC20Delta> for the given token index
    ///
    /// The ERC20Delta struct is wrapped in MaybeUninit and has an `init` flag.
    /// This format saves compute by avoiding zero-filling ERC20Delta. If `init` is false,
    /// we overwrite the ERC20Delta and set `init` to true.
    fn get_delta_mut(&mut self, token_index: TokenIndex<T>) -> &mut Lazy<ERC20Delta>;
}

impl ERC20DeltaList<HardcodedToken> for HardcodedTokenDeltas {
    fn get_delta_mut(&mut self, token_index: TokenIndex<HardcodedToken>) -> &mut Lazy<ERC20Delta> {
        &mut self[token_index.inner as usize]
    }
}

impl ERC20DeltaList<CustomToken> for CustomTokenDeltas {
    fn get_delta_mut(&mut self, token_index: TokenIndex<CustomToken>) -> &mut Lazy<ERC20Delta> {
        &mut self[token_index.inner as usize]
    }
}
