use core::mem::MaybeUninit;

use crate::{
    goblin_error::GoblinError, settlement::IndexedERC20Delta, tokens::get_token_by_index,
    types::Address,
};

use super::ERC20Delta;

// Max allowed deltas
//
// The max length of `withdrawal_list` is also 15. If withdrawal_list has 15
// elements we cannot insert more in ERC20DeltaList
pub const MAX_DELTAS: usize = 15;

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

    pub fn init(
        indexed_erc20_delta_list: &[IndexedERC20Delta],
        custom_token_list: &[Address],
    ) -> Result<Self, GoblinError> {
        let mut list = Self::default();

        for (i, item) in indexed_erc20_delta_list.iter().enumerate() {
            let address = get_token_by_index(custom_token_list, item.index as usize)?;
            let withdrawal_due = item.withdrawal_due;
            list.inner[i].write(ERC20Delta::new(address, withdrawal_due));
        }

        Ok(list)
    }

    /// Returns an iterator over the initialized ERC20Delta elements
    pub fn iter(&self) -> impl Iterator<Item = &ERC20Delta> {
        self.inner[..self.len]
            .iter()
            .map(|maybe_uninit| unsafe { maybe_uninit.assume_init_ref() })
    }

    /// Returns a mutable iterator over the initialized ERC20Delta elements
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut ERC20Delta> {
        self.inner[..self.len]
            .iter_mut()
            .map(|maybe_uninit| unsafe { maybe_uninit.assume_init_mut() })
    }
}
