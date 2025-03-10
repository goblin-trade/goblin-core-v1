use core::mem::MaybeUninit;

use crate::{
    native_keccak256,
    quantities::Lots,
    state::{slot_key::SlotKey, SlotState},
    storage_cache_bytes32, storage_load_bytes32,
    types::Address,
};

#[repr(C)]
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

#[repr(C)]
#[derive(Debug)]
pub struct TraderTokenState {
    pub lots_locked: Lots,
    pub lots_free: Lots,
    _padding: [u8; 16],
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
