use crate::{
    goblin_error::GoblinError, require, settlement::ERC20DeltaInput, tokens::TokenIndex,
    utils::FixedMap,
};

use super::ERC20Delta;

// Max allowed deltas
//
// The max length of `withdrawal_list` is also 15. If withdrawal_list has 15
// elements we cannot insert more in ERC20DeltaList
pub const MAX_DELTAS: usize = 16;

/// An expandable list of ERC20 deltas.
pub type ERC20DeltaList = FixedMap<TokenIndex, ERC20Delta, MAX_DELTAS>;

impl ERC20DeltaList {
    /// Create a new ERC20DeltaList initialized with inputs read from args
    pub fn new(erc20_delta_input_list: &[ERC20DeltaInput]) -> Result<Self, GoblinError> {
        require!(
            erc20_delta_input_list.len() <= MAX_DELTAS,
            GoblinError::ERC20DeltaListFull
        );

        let mut entries: [core::mem::MaybeUninit<(TokenIndex, ERC20Delta)>; MAX_DELTAS] =
            [const { core::mem::MaybeUninit::uninit() }; MAX_DELTAS];

        for (i, delta_input) in erc20_delta_input_list.iter().enumerate() {
            entries[i].write((
                delta_input.index,
                ERC20Delta::new(delta_input.withdrawal_due),
            ));
        }

        Ok(FixedMap::new_unchecked(
            entries,
            erc20_delta_input_list.len(),
        ))
    }
}
