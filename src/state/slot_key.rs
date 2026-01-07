use core::marker::PhantomData;

use crate::{
    hostio::{self, hostio_helpers, storage_cache_bytes32, storage_load_bytes32},
    state::{Preimage, PreimageSerializer},
};

// pub trait SlotState: Sized {
//     /// Unique 1 byte slot discriminator
//     ///
//     /// Discriminators can be standalone or derived from sub-discriminators.
//     ///
//     /// # Avoiding collisions
//     ///
//     /// * Standalone: Use 3 bits, i.e. values in [0, 7].
//     ///
//     /// * Derived discriminator
//     ///   - First sub-discriminator takes 3 bits
//     ///   - Left shift and add the other ones.
//     ///   - Eg. Market discriminator = MarketVariant::D + Base::D << 3 + Quote::D << 4.
//     const SLOT_DISCRIMINATOR: u8;
// }

pub struct SlotKey<P: Preimage> {
    hash: [u8; 32],
    _marker: PhantomData<P>,
}

impl<P: Preimage> SlotKey<P> {
    pub fn hash(&self) -> &[u8; 32] {
        &self.hash
    }

    pub const fn new_inner(hash: [u8; 32]) -> Self {
        Self {
            hash,
            _marker: PhantomData,
        }
    }

    pub fn generate(preimage: P) -> Self {
        let buffer = PreimageSerializer::new(preimage);
        let bytes = buffer.serialize();

        let hash = hostio_helpers::native_keccak256(bytes);
        Self::new_inner(hash)
    }

    // pub fn generate(bytes: &[u8]) -> Self {
    //     let hash = hostio_helpers::native_keccak256(bytes);
    //     Self::new_inner(hash)
    // }

    pub fn load(&self) -> P::SlotState {
        storage_load_bytes32::<P::SlotState>(&self.hash)
    }

    pub fn store(&self, value: &P::SlotState) {
        storage_cache_bytes32::<P::SlotState>(&self.hash, value)
    }
}
