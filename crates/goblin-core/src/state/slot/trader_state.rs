use crate::{
    native_keccak256,
    quantities::{BaseLots, QuoteLots},
    state::{slot_key::SlotKey, SlotState},
    storage_cache_bytes32, storage_load_bytes32,
    types::Address,
};

#[repr(C, packed)]
pub struct TraderTokenKey {
    pub trader: Address,
    pub token: Address,
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

#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct TraderTokenState {
    pub lots_locked: u64,
    pub lots_free: u64,
}

impl SlotState<TraderTokenKey, TraderTokenState> for TraderTokenState {
    fn load(key: &TraderTokenKey) -> &mut TraderTokenState {
        // We pass 16 and not 32 byte slot because we only need the first 16 bytes
        // The function's name is storage_load_bytes32(). Need to check if using 16 bytes works.
        let mut slot = [0u8; 16];
        unsafe {
            storage_load_bytes32(key.to_keccak256().as_ptr(), slot.as_mut_ptr());
        }

        let trader_token_state = unsafe { &mut *(slot.as_mut_ptr() as *mut TraderTokenState) };
        trader_token_state
    }

    fn store(&self, key: &TraderTokenKey) {
        unsafe {
            storage_cache_bytes32(
                key.to_keccak256().as_ptr(),
                self as *const TraderTokenState as *const u8,
            );
        }
    }
}
