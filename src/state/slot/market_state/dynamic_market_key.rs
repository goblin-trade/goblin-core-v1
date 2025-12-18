use core::marker::PhantomData;

use crate::{hostio::HostioBuffer, markets::PairShape, state::SlotKey};

/// The key for a custom market
pub struct DynamicMarketKey<P: PairShape> {
    hash: HostioBuffer<[u8; 32]>,
    _marker: PhantomData<P>,
}

impl<P: PairShape> DynamicMarketKey<P> {
    pub fn new(hash: HostioBuffer<[u8; 32]>) -> Self {
        Self {
            hash,
            _marker: PhantomData,
        }
    }
}

impl<P: PairShape> SlotKey for DynamicMarketKey<P> {
    const DISCRIMINATOR: u8 = P::DISCRIMINATOR;

    fn hash(&self) -> &[u8; 32] {
        self.hash.as_ref()
    }
}
