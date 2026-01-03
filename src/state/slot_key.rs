use core::marker::PhantomData;

use crate::hostio::{self, storage_cache_bytes32, storage_load_bytes32, HostioBuffer};

pub trait SlotState: Sized {
    // Ensure that size equals 32 bytes at compile time
    const ASSERT: () = assert!(core::mem::size_of::<Self>() == 32);

    /// Unique 1 byte discriminator
    const DISCRIMINATOR: u8;
}

pub struct SlotKey<S: SlotState> {
    hash: [u8; 32],
    _marker: PhantomData<S>,
}

impl<S: SlotState> SlotKey<S> {
    // const SIZE_CHECK: () = assert!(core::mem::size_of::<S>() == 32);

    // pub const DISCRIMINATOR: u8 = D;

    pub fn hash(&self) -> &[u8; 32] {
        self.hash()
    }

    pub const fn new_inner(hash: [u8; 32]) -> Self {
        Self {
            hash,
            _marker: PhantomData,
        }
    }

    pub fn generate(bytes: &[u8]) -> Self {
        let hash = hostio::native_keccak256(bytes);
        Self::new_inner(hash)
    }

    pub fn load(&self) -> S {
        storage_load_bytes32(&self.hash)
    }

    pub fn store(&self, value: &S) {
        storage_cache_bytes32::<S>(&self.hash, value)
    }
}

// pub trait SlotKey {
//     /// Unique 1 byte discriminator for the slot. We need different discriminators for
//     /// different slots sharing the same namespace- eg. FreeAtomState and LockedAtomState
//     const DISCRIMINATOR: u8;

//     fn hash(&self) -> &[u8; 32];
// }

// pub trait SlotState<T: SlotKey>: Sized {
//     // Ensure that size equals 32 bytes at compile time
//     const ASSERT: () = assert!(core::mem::size_of::<Self>() == 32);

//     fn load(key: &T) -> HostioBuffer<Self> {
//         storage_load_bytes32::<Self>(key.hash())
//     }

//     fn store(&self, key: &T) {
//         storage_cache_bytes32::<Self>(key.hash(), self)
//     }
// }
