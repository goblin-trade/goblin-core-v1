use core::marker::PhantomData;

use crate::{
    market::MarketVariant, quantities::Ticks, state::SlotState, token::TokenMarker, types::Pair,
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

impl<M: MarketVariant, B: TokenMarker, Q: TokenMarker> SlotState for MarketState<M, B, Q> {
    const DISCRIMINATOR: u8 = M::DISCRIMINATOR + B::DISCRIMINATOR << 1 + Q::DISCRIMINATOR << 2;
}
