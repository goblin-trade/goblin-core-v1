use core::marker::PhantomData;

use crate::{
    axis::{leg::Pair, market::market_marker::MarketMarker, token::token_marker::TokenMarker},
    quantities::Ticks,
};

/// The market state slot
/// We have 6 possible sub-types based on MarketVariant and PairShape
#[repr(C)]
pub struct MarketState<M, B, Q>
where
    M: MarketMarker,
    B: TokenMarker,
    Q: TokenMarker,
{
    /// The last known prices at the centre
    /// The best price is equal to or worse than the last price.
    pub last_prices: Pair<Ticks, Ticks>,
    /// Padding to match 32 bits
    _padding: [u8; 16],
    _marker: PhantomData<(M, B, Q)>,
}

// impl<M: MarketVariant, B: TokenMarker, Q: TokenMarker> SlotState for MarketState<M, B, Q> {
//     /// Derived discriminator. Left shift by 3 bits to avoid collision.
//     const SLOT_DISCRIMINATOR: u8 = M::DISCRIMINATOR + B::DISCRIMINATOR << 3 + Q::DISCRIMINATOR << 4;
// }

// DynamicMarketHasher is gone
// Each market type gets a pre-image
