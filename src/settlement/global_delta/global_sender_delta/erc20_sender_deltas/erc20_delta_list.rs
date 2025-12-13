use crate::{
    settlement::global_delta::{ERC20Delta, SenderCustomDeltas, SenderHardcodedDeltas},
    token::{CustomToken, ERC20Marker, HardcodedToken, TokenIndex},
};

pub const MAX_CUSTOM_DELTAS: usize = 8;

/// The list of ERC20 deltas. These deltas are stored in a fixed size list, indexed by
/// the token index.
///
/// This function just maps index to delta. It doesn't deal with the wrapper struct.
pub trait ERC20DeltaList<T: ERC20Marker> {
    /// Get a mutable reference to ERC20Delta for the given token index
    fn get_delta_mut(&mut self, token_index: TokenIndex<T>) -> &mut ERC20Delta;
}

impl ERC20DeltaList<HardcodedToken> for SenderHardcodedDeltas {
    fn get_delta_mut(&mut self, token_index: TokenIndex<HardcodedToken>) -> &mut ERC20Delta {
        &mut self[token_index.inner as usize]
    }
}

impl ERC20DeltaList<CustomToken> for SenderCustomDeltas {
    fn get_delta_mut(&mut self, token_index: TokenIndex<CustomToken>) -> &mut ERC20Delta {
        &mut self[token_index.inner as usize]
    }
}
