use core::marker::PhantomData;

use crate::{
    market::{Dynamic, Hardcoded, MarketVariant},
    quantities::Ticks,
    state::{DynamicMarketKey, HardcodedMarketKey, SlotState},
    token::TokenMarker,
    types::Pair,
};

/// The market state slot
/// We have 6 possible sub-types based on MarketVariant and PairShape
#[repr(C)]
pub struct MarketState<M: MarketVariant, B: TokenMarker, Q: TokenMarker> {
    pub best_prices: Pair<Ticks, Ticks>,
    /// Padding to match 32 bits
    _padding: [u8; 16],
    _marker: PhantomData<(M, B, Q)>,
}

impl<B: TokenMarker, Q: TokenMarker> SlotState<HardcodedMarketKey<B, Q>>
    for MarketState<Hardcoded, B, Q>
{
}
impl<B: TokenMarker, Q: TokenMarker> SlotState<DynamicMarketKey<B, Q>>
    for MarketState<Dynamic, B, Q>
{
}
