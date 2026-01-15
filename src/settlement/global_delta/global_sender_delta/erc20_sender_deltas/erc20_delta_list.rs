use crate::{
    settlement::global_delta::{ERC20Delta, SenderCustomDeltas, SenderHardcodedDeltas},
    token::{CustomERC20, ERC20Marker, HardcodedERC20, TokenIndex},
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

impl ERC20DeltaList<HardcodedERC20> for SenderHardcodedDeltas {
    fn get_delta_mut(&mut self, token_index: TokenIndex<HardcodedERC20>) -> &mut ERC20Delta {
        &mut self[token_index.inner as usize]
    }
}

impl ERC20DeltaList<CustomERC20> for SenderCustomDeltas {
    fn get_delta_mut(&mut self, token_index: TokenIndex<CustomERC20>) -> &mut ERC20Delta {
        &mut self[token_index.inner as usize]
    }
}
