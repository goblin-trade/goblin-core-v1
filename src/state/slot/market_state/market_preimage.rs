use crate::{
    axis::market::{
        market_marker::MarketMarker, market_spec::MarketSpec, token_pair::TokenPair, LotSizePair,
    },
    quantities::QuoteLotsPerBaseUnitPerTick,
    state::{MarketState, Preimage, TokenAddressPair},
};
use core::marker::PhantomData;

/// Key preimage to read MarketState from slot
///
/// This is similar to MarketState, but instead of token index pair we have
/// a pair of token addresses
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MarketPreimage<MS: MarketSpec> {
    lot_size_pair: LotSizePair,
    tick_size: QuoteLotsPerBaseUnitPerTick,
    token_address_pair: TokenAddressPair<MS::Base, MS::Quote>,
    _marker: PhantomData<MS>,
}

impl<MS: MarketSpec> MarketPreimage<MS> {
    pub fn new(
        lot_size_pair: LotSizePair,
        tick_size: QuoteLotsPerBaseUnitPerTick,
        token_address_pair: TokenAddressPair<MS::Base, MS::Quote>,
    ) -> Self {
        Self {
            lot_size_pair,
            tick_size,
            token_address_pair,
            _marker: PhantomData,
        }
    }
}

impl<MS: MarketSpec> Preimage for MarketPreimage<MS> {
    const SLOT_DISCRIMINATOR: u8 = MS::Market::DISCRIMINATOR + MS::Pair::DISCRIMINATOR;

    type SlotState = MarketState;
}
