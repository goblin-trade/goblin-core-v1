use crate::{
    axis::market::market_marker::MarketMarker,
    axis_helpers::{MarketSpec, TokenPair},
    market::{LotSizePair, TokenAddressPair},
    quantities::QuoteLotsPerBaseUnitPerTick,
    state::{MarketState, Preimage},
};
use core::marker::PhantomData;

/// Key preimage to read MarketState from slot
///
/// This is similar to MarketState, but instead of token index pair we have
/// a pair of token addresses
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MarketPreimage<TP: TokenPair> {
    lot_size_pair: LotSizePair,
    tick_size: QuoteLotsPerBaseUnitPerTick,
    token_address_pair: TokenAddressPair<TP>,
}

impl<TP: TokenPair> MarketPreimage<TP> {
    pub fn new(
        lot_size_pair: LotSizePair,
        tick_size: QuoteLotsPerBaseUnitPerTick,
        token_address_pair: TokenAddressPair<TP>,
    ) -> Self {
        Self {
            lot_size_pair,
            tick_size,
            token_address_pair,
        }
    }
}

impl<TP: TokenPair> Preimage for MarketPreimage<TP> {
    const SLOT_DISCRIMINATOR: u8 = TP::DISCRIMINATOR;

    type SlotState = MarketState;
}
