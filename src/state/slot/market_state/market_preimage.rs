use core::marker::PhantomData;

use crate::{
    market::{LotSizePair, MarketVariant},
    quantities::QuoteLotsPerBaseUnitPerTick,
    state::{MarketState, Preimage},
    token::TokenMarker,
    types::Pair,
};

/// Key preimage to read MarketState from slot
///
/// This is similar to MarketState, but instead of token index pair we have
/// a pair of token addresses
#[repr(C)]
pub struct MarketPreimage<M: MarketVariant, B: TokenMarker, Q: TokenMarker> {
    lot_size_pair: LotSizePair,
    tick_size: QuoteLotsPerBaseUnitPerTick,
    token_address_pair: Pair<B::Address, Q::Address>,
    _marker: PhantomData<M>,
}

impl<M: MarketVariant, B: TokenMarker, Q: TokenMarker> MarketPreimage<M, B, Q> {
    pub fn new(
        lot_size_pair: LotSizePair,
        tick_size: QuoteLotsPerBaseUnitPerTick,
        token_address_pair: Pair<B::Address, Q::Address>,
    ) -> Self {
        Self {
            lot_size_pair,
            tick_size,
            token_address_pair,
            _marker: PhantomData,
        }
    }
}

impl<M: MarketVariant, B: TokenMarker, Q: TokenMarker> Preimage for MarketPreimage<M, B, Q> {
    const SLOT_DISCRIMINATOR: u8 = M::DISCRIMINATOR + B::DISCRIMINATOR << 3 + Q::DISCRIMINATOR << 4;

    type SlotState = MarketState<M, B, Q>;
}
