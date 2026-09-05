use core::marker::PhantomData;

use crate::{
    hostio::{storage_cache_bytes32, storage_load_bytes32},
    state::Preimage,
};

/// The slot key obtained on hashing a preimage
#[derive(Clone, Copy, Default)]
pub struct SlotKey<P: Preimage> {
    hash: [u8; 32],
    _marker: PhantomData<P>,
}

impl<P: Preimage> SlotKey<P> {
    pub const fn new(hash: [u8; 32]) -> Self {
        Self {
            hash,
            _marker: PhantomData,
        }
    }

    pub fn hash(&self) -> &[u8; 32] {
        &self.hash
    }

    pub fn load(&self) -> P::SlotState {
        storage_load_bytes32::<P::SlotState>(&self.hash)
    }

    pub fn store(&self, value: &P::SlotState) {
        storage_cache_bytes32::<P::SlotState>(&self.hash, value)
    }
}
