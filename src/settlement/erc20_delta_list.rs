use core::mem::MaybeUninit;

use crate::{
    goblin_error::GoblinError, quantities::AtomsDelta, settlement::ERC20DeltaInput,
    tokens::TokenIndex,
};

use super::ERC20Delta;

// Max allowed deltas
//
// The max length of `withdrawal_list` is also 15. If withdrawal_list has 15
// elements we cannot insert more in ERC20DeltaList
pub const MAX_DELTAS: usize = 15;

/// An expandable list of ERC20 deltas.
/// It gets initialized with `indexed_erc20_delta_list` from args. However more tokens
/// can be added as we perform trades.
pub struct ERC20DeltaList {
    /// The list of token deltas
    inner: [MaybeUninit<ERC20Delta>; MAX_DELTAS],

    /// Gives the number of tokens being tracked. Rest of the elements hold default values.
    pub len: usize,
}

impl ERC20DeltaList {
    fn default() -> Self {
        ERC20DeltaList {
            inner: [MaybeUninit::<ERC20Delta>::uninit(); MAX_DELTAS],
            len: 0,
        }
    }

    pub fn init(erc20_delta_input_list: &[ERC20DeltaInput]) -> Result<Self, GoblinError> {
        let mut list = Self::default();
        list.len = erc20_delta_input_list.len();

        for (i, item) in erc20_delta_input_list.iter().enumerate() {
            list.inner[i].write(ERC20Delta::new(item.index, item.withdrawal_due));
        }

        Ok(list)
    }

    /// Returns an iterator over the initialized ERC20Delta elements
    pub fn iter(&self) -> impl Iterator<Item = &ERC20Delta> {
        self.inner[..self.len]
            .iter()
            .map(|maybe_uninit| unsafe { maybe_uninit.assume_init_ref() })
    }

    /// Returns an iterator over the initialized ERC20Delta elements
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut ERC20Delta> {
        self.inner[..self.len]
            .iter_mut()
            .map(|maybe_uninit| unsafe { maybe_uninit.assume_init_mut() })
    }

    /// Returns a mutable reference to the ERC20Delta for the given token index.
    /// If the token index is not found, inserts a new ERC20Delta with zero deltas.
    ///
    /// Validity check of the token index is deferred to the settlement phase.
    pub fn get_or_insert(
        &mut self,
        token_index: TokenIndex,
    ) -> Result<&mut ERC20Delta, GoblinError> {
        // First, try to find existing delta with this token index
        for i in 0..self.len {
            let delta_ref = unsafe { self.inner[i].assume_init_ref() };
            if delta_ref.index == token_index {
                return Ok(unsafe { self.inner[i].assume_init_mut() });
            }
        }

        // Not found, insert new delta
        if self.len >= MAX_DELTAS {
            return Err(GoblinError::DeltaListFull);
        }

        // Insert new delta at the end
        let new_delta = ERC20Delta::new(token_index, AtomsDelta::ZERO);
        self.inner[self.len].write(new_delta);
        self.len += 1;

        // Return reference to the newly inserted delta
        Ok(unsafe { self.inner[self.len - 1].assume_init_mut() })
    }

    /// Returns a reference to the ERC20Delta for the given token index.
    /// Returns None if the token index is not found.
    pub fn get(&self, token_index: TokenIndex) -> Option<&ERC20Delta> {
        for i in 0..self.len {
            let delta_ref = unsafe { self.inner[i].assume_init_ref() };
            if delta_ref.index == token_index {
                return Some(delta_ref);
            }
        }
        None
    }
}
