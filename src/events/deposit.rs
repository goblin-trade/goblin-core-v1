use core::mem::MaybeUninit;

use crate::{
    quantities::Atoms,
    state::{SlotState, TraderTokenKey, TraderTokenState},
};

#[cfg(all(not(test), not(target_arch = "wasm32")))]
use crate::indexer_hostio;

pub fn deposit(key: &TraderTokenKey, atoms: Atoms, decimals: u8) {
    let mut trader_token_state_maybe = MaybeUninit::<TraderTokenState>::uninit();
    let trader_token_state = unsafe { TraderTokenState::load(key, &mut trader_token_state_maybe) };
    trader_token_state.atoms_free += atoms;
    trader_token_state.decimals = decimals;

    unsafe {
        trader_token_state.store(key);

        #[cfg(all(not(test), not(target_arch = "wasm32")))]
        indexer_hostio::index_deposit(key.trader.as_ptr(), key.token.as_ptr(), atoms.0);
    }
}
