use core::mem::MaybeUninit;

use crate::{
    quantities::Lots,
    state::{SlotState, TraderTokenKey, TraderTokenState},
};

#[cfg(all(not(test), not(target_arch = "wasm32")))]
use crate::indexer_hostio;

pub fn deposit(key: &TraderTokenKey, lots: Lots) {
    let mut trader_token_state_maybe = MaybeUninit::<TraderTokenState>::uninit();
    let trader_token_state = unsafe { TraderTokenState::load(key, &mut trader_token_state_maybe) };
    trader_token_state.lots_free += lots;

    unsafe {
        trader_token_state.store(key);

        #[cfg(all(not(test), not(target_arch = "wasm32")))]
        indexer_hostio::index_deposit(key.trader.as_ptr(), key.token.as_ptr(), lots.0);
    }
}
