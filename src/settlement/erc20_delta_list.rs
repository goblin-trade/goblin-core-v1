use core::mem::MaybeUninit;

use crate::{
    goblin_error::GoblinError, quantities::Delta, tokens::get_token_by_index, types::Address,
};

use super::ERC20Delta;

// Max allowed deltas
//
// The max length of `withdrawal_list` is also 15. If withdrawal_list has 15
// elements we cannot insert more in ERC20DeltaList
pub const MAX_DELTAS: usize = 15;

// Number of bytes per token in erc20_withdrawals_bytes
pub const ERC20_WITHDRAWAL_ITEM_SIZE: usize = 9;

pub struct ERC20DeltaList {
    /// The list of token deltas
    pub deltas: [MaybeUninit<ERC20Delta>; MAX_DELTAS],

    /// Gives the number of tokens being tracked. Rest of the elements hold default values.
    pub len: usize,
}

impl ERC20DeltaList {
    fn default() -> Self {
        ERC20DeltaList {
            deltas: [MaybeUninit::<ERC20Delta>::uninit(); MAX_DELTAS],
            len: 0,
        }
    }

    pub fn init(bytes: &[u8], custom_token_list: &[Address]) -> Result<Self, GoblinError> {
        debug_assert!(bytes.len() <= MAX_DELTAS * ERC20_WITHDRAWAL_ITEM_SIZE);

        let mut delta_list = ERC20DeltaList::default();
        delta_list.len = bytes.len() / ERC20_WITHDRAWAL_ITEM_SIZE;

        // TODO optimize with as_chunks() when stable
        // let (chunks, _remainder): (&[[u8; 9]], &[u8]) = bytes.as_chunks();
        for (i, chunk) in bytes.chunks_exact(ERC20_WITHDRAWAL_ITEM_SIZE).enumerate() {
            let index = chunk[0];
            let address = get_token_by_index(custom_token_list, index as usize)?;
            let withdrawal_due = Delta(unsafe { *(chunk.as_ptr().add(1) as *const i64) });
            delta_list.deltas[i].write(ERC20Delta::new(index, address, withdrawal_due));
        }

        Ok(delta_list)
    }
}
