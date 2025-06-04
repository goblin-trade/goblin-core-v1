use core::mem::MaybeUninit;

use crate::{
    erc20,
    goblin_error::GoblinError,
    quantities::Delta,
    state::{SlotState, TraderTokenKey, TraderTokenState},
    tokens::get_token_by_index,
    types::Address,
};

/// Tracks tokens that must be deposited in or withdrawn on settlement
///
/// This is indepenent of tokens used by or emitted from matching. They are tracked separately
/// in an array of TokensConsumedByEngine.
///
/// Such tokens don't get deposited or withdrawn, but only adjusted from TraderTokenState.
///However if `deposit_shortfall` is true then it must be transferred in.
///
/// * If token is present in the list, add shortfall here
/// * Otherwise transfer shortfall directly based on TokensConsumedByEngine. We cannot insert new elements in this list.
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
        deposit_shortfall: bool,
        withdraw_internally: bool,
    ) -> Result<(), GoblinError> {
        let delta = self.delta;

        if delta == Delta::ZERO {
            return Ok(());
        }
        let token_address = get_token_by_index(custom_token_list, self.index as usize)?;
        Ok(())
    }

    fn settle_for_sender(
        &self,
        msg_sender: &Address,
        token_address: &Address,
        deposit_shortfall: bool,
    ) -> Result<(), GoblinError> {
        let key = &TraderTokenKey {
            token: *token_address,
            trader: *msg_sender,
        };
        let mut trader_token_state_maybe = MaybeUninit::<TraderTokenState>::uninit();
        let trader_token_state =
            unsafe { TraderTokenState::load(key, &mut trader_token_state_maybe) };

        let delta = self.delta;
        let atoms = delta.abs();

        // Subtract positive delta from TTS
        // If TTS cannot absorb delta, fail unless deposit_shortfall is true.
        // In deposit_shortfall case, reduce delta
        if delta > Delta::ZERO {}
        Ok(())
    }
}
