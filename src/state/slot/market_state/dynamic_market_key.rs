use core::marker::PhantomData;

use crate::{
    goblin_error::GoblinError,
    hostio::{self, HostioBuffer},
    markets::CommonMarket,
    state::SlotKey,
    token::{CustomToken, DynamicIndex, TokenMarker, ERC20, ETH},
    types::{Base, Quote, TupleReader},
};

/// The key for a custom market
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

    pub fn set_common_fields<const N: usize>(
        bytes: &mut [u8; N],
        market: &CommonMarket<DynamicIndex, B, Q>,
    ) {
        bytes[0] = Self::DISCRIMINATOR;

        bytes[1..9].copy_from_slice(&Base::get(&market.lot_size_pair).inner.to_le_bytes());
        bytes[9..17].copy_from_slice(&Quote::get(&market.lot_size_pair).inner.to_le_bytes());
        bytes[17..25].copy_from_slice(&market.tick_size.inner.to_le_bytes());
    }
}

impl<B: TokenMarker, Q: TokenMarker> SlotKey for DynamicMarketKey<B, Q> {
    const DISCRIMINATOR: u8 = B::DISCRIMINATOR + Q::DISCRIMINATOR << 1;

    fn hash(&self) -> &[u8; 32] {
        self.hash.as_ref()
    }
}
