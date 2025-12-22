use core::marker::PhantomData;

use crate::{hostio::HostioBuffer, state::SlotKey, token::TokenMarker};

/// Slot key for a dynamic market
///
/// It is generated with DynamicMarketHasher trait
pub struct DynamicMarketKey<B: TokenMarker, Q: TokenMarker> {
    hash: HostioBuffer<[u8; 32]>,
    _marker: PhantomData<(B, Q)>,
}

impl<B: TokenMarker, Q: TokenMarker> DynamicMarketKey<B, Q> {
    pub fn new(hash: HostioBuffer<[u8; 32]>) -> Self {
        Self {
            hash,
            _marker: PhantomData,
        }
    }
}

impl<B: TokenMarker, Q: TokenMarker> SlotKey for DynamicMarketKey<B, Q> {
    const DISCRIMINATOR: u8 = B::DISCRIMINATOR + Q::DISCRIMINATOR << 1;

    fn hash(&self) -> &[u8; 32] {
        self.hash.as_ref()
    }
}
