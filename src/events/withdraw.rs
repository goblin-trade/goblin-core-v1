use core::mem::MaybeUninit;

use crate::{
    quantities::Lots,
    state::{SlotState, TraderTokenKey, TraderTokenState},
};

pub fn withdraw(key: &TraderTokenKey, lots: Lots) -> Lots {
    let mut trader_token_state_maybe = MaybeUninit::<TraderTokenState>::uninit();
    let trader_token_state = unsafe { TraderTokenState::load(key, &mut trader_token_state_maybe) };

    let lots_to_withdraw = lots.min(trader_token_state.lots_free);
    trader_token_state.lots_free = lots_to_withdraw;

    unsafe {
        trader_token_state.store(key);

        // TODO indexer_hostio
    }

    lots_to_withdraw
}
