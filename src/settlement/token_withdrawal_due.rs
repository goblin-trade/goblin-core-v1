use crate::{
    goblin_error::GoblinError, quantities::Delta, tokens::get_token_by_index, types::Address,
};

/// Tracks tokens that must be deposited in or withdrawn on settlement
///
/// This is indepenent of tokens used by or emitted from matching. They are tracked separately.
/// Such tokens don't get deposited or withdrawn, but only adjusted from TraderTokenState.
///
/// However if `deposit_shortfall` is true then it must be transferred in. We can
///
#[repr(C, packed)]
pub struct TokenWithdrawalDue {
    /// The token index
    pub index: u8,

    /// The amount pending withdrawal
    pub delta: Delta,
}

impl TokenWithdrawalDue {
    pub fn settle(
        &self,
        custom_token_list: &[Address],
        msg_sender: &Address,
        recipient: &Address,
    ) -> Result<(), GoblinError> {
        let token_address = get_token_by_index(custom_token_list, self.index as usize)?;
        Ok(())
    }
}
