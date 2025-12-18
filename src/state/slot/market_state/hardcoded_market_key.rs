use core::marker::PhantomData;

use crate::{markets::PairShape, state::SlotKey};

/// The hash is hardcoded for hardcoded markets
pub struct HardcodedMarketKey<P: PairShape> {
    hash: [u8; 32],
    _marker: PhantomData<P>,
}

impl<P: PairShape> HardcodedMarketKey<P> {
    pub const fn new(hash: [u8; 32]) -> Self {
        Self {
            hash,
            _marker: PhantomData,
        }
    }
}

impl<P: PairShape> SlotKey for HardcodedMarketKey<P> {
    // The PairShape discriminator not used at runtime. But it is used for
    // pre-computing the hash for hardcoding.
    const DISCRIMINATOR: u8 = P::DISCRIMINATOR;

    fn hash(&self) -> &[u8; 32] {
        &self.hash
    }
}
