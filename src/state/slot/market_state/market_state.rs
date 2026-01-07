use core::marker::PhantomData;

use crate::{market::MarketVariant, quantities::Ticks, token::TokenMarker, types::Pair};

/// The market state slot
/// We have 6 possible sub-types based on MarketVariant and PairShape
#[repr(C)]
pub struct MarketState<M: MarketVariant, B: TokenMarker, Q: TokenMarker> {
    pub best_prices: Pair<Ticks, Ticks>,
    /// Padding to match 32 bits
    _padding: [u8; 16],
    _marker: PhantomData<(M, B, Q)>,
}

// impl<M: MarketVariant, B: TokenMarker, Q: TokenMarker> SlotState for MarketState<M, B, Q> {
//     /// Derived discriminator. Left shift by 3 bits to avoid collision.
//     const SLOT_DISCRIMINATOR: u8 = M::DISCRIMINATOR + B::DISCRIMINATOR << 3 + Q::DISCRIMINATOR << 4;
// }
