use crate::{goblin_error::GoblinError, types::Address};

const MAX_DELTAS: usize = 16;

/// The amount of tokens that must be transferred on settlement
#[derive(Default)]
pub struct TokenDelta {
    /// The token address
    pub address: Address,

    /// atoms due to be deducted from TraderTokenState (slot) on settlement
    ///
    /// * Positive: Deduct from TraderTokenState
    /// * Negative: add to TraderTokenState
    ///
    /// When tokens are used up to place orders, increase the delta. This delta
    /// must be squared off from TraderTokenState. Conversely if delta is negative,
    /// square off by crediting atoms to TraderTokenState
    ///
    /// TraderTokenState should have sufficient balance to cover slot_deduction_due
    /// on settlement, else the TX will revert due to insufficient funds.
    pub slot_deduction_due: i64,

    /// atoms due to be transferred out to trader's ERC20 account on settlement
    pub erc20_withdrawal_due: i64,
}

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

impl TokenDelta {
    /// Prepare `amount` for withdrawal. Pass negative value of `amount` for deposits
    pub fn execute_withdraw(&mut self, amount: i64) {
        self.slot_deduction_due += amount;
        self.erc20_withdrawal_due += amount;
    }

    pub fn settle(&mut self) {
        // TODO ensure that TraderTokenState can cover slot_deduction_due
        // TODO transfer out erc20_withdrawal_due
    }
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
            self.deltas[i] = TokenDelta {
                address: token,
                slot_deduction_due: 0,
                erc20_withdrawal_due: 0,
            };
            self.len += 1;
            return Ok(&mut self.deltas[i]);
        }

        // No space left
        Err(GoblinError::DeltaListFull)
    }
}
