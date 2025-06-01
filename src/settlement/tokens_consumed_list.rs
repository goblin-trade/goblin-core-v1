use crate::{goblin_error::GoblinError, types::Address};

use super::{TokenWithdrawalDue, TokensConsumedByEngine};

pub const MAX_DELTAS: usize = 16;

/// Token deltas consumed during matching. Subtract these deltas from TraderTokenState during settlement.
///
/// If deposit_shortfall is true and TraderTokenState cannot cover the delta
/// - If token is present in TokenWithdrawalDue[], add shortfall there.
/// - Otherwise transfer in the shortfall directly
pub struct TokensConsumedList {
    /// The list of token deltas
    pub deltas: [TokensConsumedByEngine; MAX_DELTAS],

    /// Gives the number of tokens being tracked. Rest of the elements hold default values.
    pub len: usize,
}

impl TokensConsumedList {
    pub fn new() -> Self {
        TokensConsumedList {
            deltas: [TokensConsumedByEngine::default(); MAX_DELTAS],
            len: 0,
        }
    }
}

// impl TokenDeltaList {
//     // Try to get TokenDelta entry for a given token. If no entry exists, it is created.
//     // If the delta list is out of capacity then this returns an Error.
//     //
//     // Prefer linear looping over binary search because elements will be less than 8
//     // in most cases
//     //
//     // Lookup in reverse order from last element to first. This way instructions
//     // can be efficiently built to minimize looping by grouping markets with
//     // common tokens together.
//     //
//     pub fn get(&mut self, token: Address) -> Result<&mut TokenDelta, GoblinError> {
//         // First, try to find an existing entry
//         for i in (0..self.len).rev() {
//             if self.deltas[i].address == token {
//                 return Ok(&mut self.deltas[i]);
//             }
//         }

//         // If not found, insert new entry if there's space
//         if self.len < MAX_DELTAS {
//             let i = self.len;
//             self.deltas[i].address = token;
//             self.len += 1;
//             return Ok(&mut self.deltas[i]);
//         }

//         // No space left
//         Err(GoblinError::DeltaListFull)
//     }
// }
