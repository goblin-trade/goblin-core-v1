use core::{
    mem::MaybeUninit,
    ops::{Add, Sub},
};

use crate::{
    goblin_error::GoblinError,
    native_keccak256,
    quantities::{Atoms, Delta},
    require,
    settlement::EthDelta,
    state::{slot_key::SlotKey, SlotState},
    storage_cache_bytes32, storage_load_bytes32,
    types::{Address, NATIVE_TOKEN},
};

#[repr(C)]
pub struct TraderTokenKey {
    pub trader: Address,
    pub token: Address,
}

impl TraderTokenKey {
    pub fn native_key(trader: &Address) -> Self {
        TraderTokenKey {
            trader: *trader,
            token: NATIVE_TOKEN,
        }
    }
}

impl SlotKey for TraderTokenKey {
    fn discriminator() -> u8 {
        0
    }

    fn to_keccak256(&self) -> [u8; 32] {
        let mut key = [0u8; 32];

        let bytes = {
            let mut b = [0u8; core::mem::size_of::<Self>() + 1];
            b[0] = Self::discriminator();
            b[1..21].copy_from_slice(&self.trader);
            b[21..41].copy_from_slice(&self.token);
            b
        };

        unsafe {
            native_keccak256(
                bytes.as_ptr(),
                core::mem::size_of::<Self>() + 1,
                key.as_mut_ptr(),
            );
        }

        key
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct TraderTokenState {
    pub atoms_locked: Atoms,
    pub atoms_free: Atoms,

    /// Number of decimal places in the token
    pub decimals: u8,
    _padding: [u8; 15],
}

impl TraderTokenState {
    pub fn new(atoms_locked: Atoms, atoms_free: Atoms, decimals: u8) -> Self {
        TraderTokenState {
            atoms_locked,
            atoms_free,
            decimals,
            _padding: [0u8; 15],
        }
    }

    pub fn is_empty(&self) -> bool {
        self.atoms_locked == Atoms::ZERO && self.atoms_free == Atoms::ZERO && self.decimals == 0
    }

    pub fn add_free_atoms_and_store(key: &TraderTokenKey, atoms: Atoms, decimals: u8) {
        let mut trader_token_state_maybe = MaybeUninit::<TraderTokenState>::uninit();
        let trader_token_state =
            unsafe { TraderTokenState::load(key, &mut trader_token_state_maybe) };

        trader_token_state.atoms_free += atoms;
        trader_token_state.decimals = decimals;
        unsafe {
            trader_token_state.store(key);
        }
    }
}

impl SlotState<TraderTokenKey, TraderTokenState> for TraderTokenState {
    unsafe fn load<'a>(
        key: &TraderTokenKey,
        slot: &'a mut MaybeUninit<TraderTokenState>,
    ) -> &'a mut TraderTokenState {
        storage_load_bytes32(key.to_keccak256().as_ptr(), slot.as_mut_ptr() as *mut u8);
        slot.assume_init_mut()
    }

    unsafe fn store(&self, key: &TraderTokenKey) {
        storage_cache_bytes32(
            key.to_keccak256().as_ptr(),
            self as *const TraderTokenState as *const u8,
        );
    }
}

#[cfg(test)]
mod test {
    use crate::state::SlotKey;

    use super::TraderTokenKey;

    #[test]
    fn get_keccak() {
        let key = TraderTokenKey {
            trader: [0u8; 20],
            token: [0u8; 20],
        };

        let hash = key.to_keccak256();
        println!("{:?}", hash);
    }
}
