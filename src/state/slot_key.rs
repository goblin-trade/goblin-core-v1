use core::mem::MaybeUninit;

use crate::hostio::{hostio_storage_cache_bytes32, hostio_storage_load_bytes32, HostioBuffer};

pub trait SlotKey {
    /// Unique 1 byte discriminator for the slot. We need different discriminators for
    /// different slots sharing the same namespace- eg. FreeAtomState and LockedAtomState
    fn discriminator() -> u8;

    fn to_keccak256(&self) -> [u8; 32];
}

pub trait SlotKeyV2 {
    /// Unique 1 byte discriminator for the slot. We need different discriminators for
    /// different slots sharing the same namespace- eg. FreeAtomState and LockedAtomState
    const DISCRIMINATOR: u8;

    fn hash(&self) -> &[u8; 32];
}

pub trait SlotStateV2<T: SlotKeyV2>: Sized {
    fn load(key: &T) -> HostioBuffer<Self> {
        unsafe { hostio_storage_load_bytes32::<Self>(key.hash()) }
    }

    fn store(&self, key: &T) {
        unsafe {
            hostio_storage_cache_bytes32::<Self>(key.hash(), self);
        }
    }
}

pub trait SlotState<K: SlotKey, S> {
    unsafe fn load<'a>(key: &K, slot: &'a mut MaybeUninit<S>) -> &'a mut S;

    unsafe fn store(&self, key: &K);
}
