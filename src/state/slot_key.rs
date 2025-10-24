use crate::hostio::{storage_cache_bytes32, storage_load_bytes32, HostioBuffer};

pub trait SlotKey {
    /// Unique 1 byte discriminator for the slot. We need different discriminators for
    /// different slots sharing the same namespace- eg. FreeAtomState and LockedAtomState
    const DISCRIMINATOR: u8;

    fn hash(&self) -> &[u8; 32];
}

pub trait SlotState<T: SlotKey>: Sized {
    // Ensure that size equals 32 bytes at compile time
    const ASSERT: () = assert!(core::mem::size_of::<Self>() == 32);

    fn load(key: &T) -> HostioBuffer<Self> {
        storage_load_bytes32::<Self>(key.hash())
    }

    fn store(&self, key: &T) {
        storage_cache_bytes32::<Self>(key.hash(), self)
    }
}
