use crate::{goblin_error::GoblinError, types::Address};

use super::TokenDelta;

const MAX_DELTAS: usize = 16;

/// The list of token deltas being tracked.
/// We cannot use hashmap in no_std, no allocator environment. Using linear looping
/// is performant enough for small number of items.
#[derive(Default)]
pub struct TokenDeltaList {
    /// The list of token deltas
    pub deltas: [TokenDelta; MAX_DELTAS],

    /// Gives the number of tokens being tracked. Rest of the elements hold default values.
    pub len: usize,
}

impl TokenDeltaList {
    // Try to get TokenDelta entry for a given token. If no entry exists, it is created.
    // If the delta list is out of capacity then this returns an Error.
    //
    // Prefer linear looping over binary search because elements will be less than 8
    // in most cases
    //
    // Lookup in reverse order from last element to first. This way instructions
    // can be efficiently built to minimize looping by grouping markets with
    // common tokens together.
    //
    pub fn get(&mut self, token: Address) -> Result<&mut TokenDelta, GoblinError> {
        // First, try to find an existing entry
        for i in (0..self.len).rev() {
            if self.deltas[i].address == token {
                return Ok(&mut self.deltas[i]);
            }
        }

        // If not found, insert new entry if there's space
        if self.len < MAX_DELTAS {
            let i = self.len;
            self.deltas[i].address = token;
            self.len += 1;
            return Ok(&mut self.deltas[i]);
        }

        // No space left
        Err(GoblinError::DeltaListFull)
    }
}
