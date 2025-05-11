use core::mem::MaybeUninit;

use crate::{
    goblin_error::GoblinError,
    quantities::{Atoms, RawAtoms},
    require,
    state::{SlotState, TraderTokenKey, TraderTokenState},
};

#[cfg(all(not(test), not(target_arch = "wasm32")))]
use crate::indexer_hostio;

pub fn withdraw(key: &TraderTokenKey, atoms: Atoms) -> Result<RawAtoms, GoblinError> {
    let mut trader_token_state_maybe = MaybeUninit::<TraderTokenState>::uninit();
    let trader_token_state = unsafe { TraderTokenState::load(key, &mut trader_token_state_maybe) };

    require!(
        !trader_token_state.is_empty(),
        GoblinError::TraderTokenStateEmpty
    );

    let atoms_to_withdraw = atoms.min(trader_token_state.atoms_free);
    trader_token_state.atoms_free -= atoms_to_withdraw;

    unsafe {
        trader_token_state.store(key);

        #[cfg(all(not(test), not(target_arch = "wasm32")))]
        indexer_hostio::index_withdraw(key.trader.as_ptr(), key.token.as_ptr(), atoms.0);
    }

    atoms_to_withdraw.to_raw_atoms(trader_token_state.decimals)
}
