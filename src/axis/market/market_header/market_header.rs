use core::marker::PhantomData;

use crate::axis::{
    leg::Pair, market::market_marker::MarketMarker, token::token_marker::TokenMarker,
};

pub struct MarketHeader<M, B, Q>
where
    M: MarketMarker,
    B: TokenMarker,
    Q: TokenMarker,
{
    /// Whether to decode deposit amounts
    pub decode_deposit_amounts: bool,

    /// Whether to execute take orders for sides In=Base and In=Quote
    pub execute_takes: Pair<bool, bool>,

    /// Number of outer bitmap indices
    pub outer_bitmap_indices: u8,

    _marker: PhantomData<(M, B, Q)>,
}

impl<M, B, Q> MarketHeader<M, B, Q>
where
    M: MarketMarker,
    B: TokenMarker,
    Q: TokenMarker,
{
    pub fn new(
        decode_deposit_amounts: bool,
        execute_takes: Pair<bool, bool>,
        outer_bitmap_indices: u8,
    ) -> Self {
        Self {
            decode_deposit_amounts,
            execute_takes,
            outer_bitmap_indices,
            _marker: PhantomData,
        }
    }
}
