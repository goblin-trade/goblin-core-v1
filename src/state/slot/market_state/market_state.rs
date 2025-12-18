use core::marker::PhantomData;

use crate::{
    markets::{MarketVariant, PairShape},
    quantities::Ticks,
    state::{DynamicMarketKey, HardcodedMarketKey, SlotState},
    token::{DynamicIndex, HardcodedToken, TokenIndex},
    types::Pair,
};

/// The market state slot
/// We have 6 possible sub-types based on MarketVariant and PairShape
#[repr(C)]
pub struct MarketState<M: MarketVariant, P: PairShape> {
    pub best_prices: Pair<Ticks, Ticks>,
    /// Padding to match 32 bits
    _padding: [u8; 16],
    _marker: PhantomData<(M, P)>,
}

impl<P: PairShape> SlotState<HardcodedMarketKey<P>> for MarketState<TokenIndex<HardcodedToken>, P> {}
impl<P: PairShape> SlotState<DynamicMarketKey<P>> for MarketState<DynamicIndex, P> {}
