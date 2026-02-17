use core::marker::PhantomData;

use crate::{
    axis::{market::market_marker::MarketMarker, token::token_marker::TokenMarker},
    quantities::BaseLots,
    types::Address,
};

/// A resting order stored in slot
/// Total size = 24 + 8 = 32. 20 byte address is padded to 24.
///
/// # Definition
///
/// * A resting order to buy or sell the given number of base lots at the given price.
///
/// * For base in (Ask)- the maker locks in `size: BaseLots` and expects to get
/// `quote lots = size * price`
///
/// * For quote in (Bid)- the maker locks in `quote lots = size * price` and expects
/// to get `size: BaseLots`
///
/// # Intermediary units for taker
///
/// * As the stored quantity is base lots, the matching unit for base in (ask) is BaseLots.
/// * If quote in case (bid), we use adjustedQuoteLots = quote lots * BaseLotsPerBaseUnit
/// * Base in taker (ask) is matched against quote in maker (bid).
#[repr(C)]
pub struct RestingOrder<M, B, Q>
where
    M: MarketMarker,
    B: TokenMarker,
    Q: TokenMarker,
{
    pub maker: Address,
    pub size: BaseLots,
    _marker: PhantomData<(M, B, Q)>,
}

impl<M, B, Q> RestingOrder<M, B, Q>
where
    M: MarketMarker,
    B: TokenMarker,
    Q: TokenMarker,
{
    pub const fn new(maker: Address, size: BaseLots) -> Self {
        Self {
            maker,
            size,
            _marker: PhantomData,
        }
    }
}
