use core::mem::MaybeUninit;

use crate::hostio::{hostio_storage_cache_bytes32, hostio_storage_load_bytes32, HostioBuffer};

pub trait SlotKey {
    /// Unique 1 byte discriminator for the slot. We need different discriminators for
    /// different slots sharing the same namespace- eg. FreeAtomState and LockedAtomState
    fn discriminator() -> u8;

    fn to_keccak256(&self) -> [u8; 32];
}

pub trait SlotKeyV2<T> {
    /// Unique 1 byte discriminator for the slot. We need different discriminators for
    /// different slots sharing the same namespace- eg. FreeAtomState and LockedAtomState
    const DISCRIMINATOR: u8;

    fn to_keccak256(&self) -> HostioBuffer<[u8; 32]>;

    fn read_slot(&self) -> HostioBuffer<T> {
        unsafe { hostio_storage_load_bytes32(self.to_keccak256().as_ref()) }
    }

    fn write_slot(&self, slot: &T) {
        unsafe { hostio_storage_cache_bytes32::<T>(self.to_keccak256().as_ref(), slot) }
    }
}

pub trait SlotState<K: SlotKey, S> {
    unsafe fn load<'a>(key: &K, slot: &'a mut MaybeUninit<S>) -> &'a mut S;

    unsafe fn store(&self, key: &K);
}
