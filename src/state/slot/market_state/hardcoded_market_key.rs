use core::marker::PhantomData;

use crate::{state::SlotKey, token::TokenMarker};

/// The hash is hardcoded for hardcoded markets
pub struct HardcodedMarketKey<B: TokenMarker, Q: TokenMarker> {
    hash: [u8; 32],
    _marker: PhantomData<(B, Q)>,
}

impl<B: TokenMarker, Q: TokenMarker> HardcodedMarketKey<B, Q> {
    pub const fn new(hash: [u8; 32]) -> Self {
        Self {
            hash,
            _marker: PhantomData,
        }
    }
}

impl<B: TokenMarker, Q: TokenMarker> SlotKey for HardcodedMarketKey<B, Q> {
    // The PairShape discriminator not used at runtime. But it is used for
    // pre-computing the hash for hardcoding.
    const DISCRIMINATOR: u8 = B::DISCRIMINATOR + Q::DISCRIMINATOR << 1;

    fn hash(&self) -> &[u8; 32] {
        &self.hash
    }
}
