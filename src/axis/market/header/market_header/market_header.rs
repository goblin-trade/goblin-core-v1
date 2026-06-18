use core::marker::PhantomData;

use crate::axis::{
    leg::SamePair, market::market_marker::MarketMarker, token::token_reader::TokenReader,
};

pub struct MarketHeader<M, B, Q>
where
    M: MarketMarker,
    B: TokenReader,
    Q: TokenReader,
{
    /// Whether to decode deposit amounts
    pub decode_deposit_amounts: bool,

    /// Whether to execute take orders for sides In=Base and In=Quote
    pub execute_takes: SamePair<bool>,

    /// Number of outer bitmaps to traverse
    pub outer_bitmap_count: u8,

    _marker: PhantomData<(M, B, Q)>,
}

impl<M, B, Q> MarketHeader<M, B, Q>
where
    M: MarketMarker,
    B: TokenReader,
    Q: TokenReader,
{
    pub fn new(
        decode_deposit_amounts: bool,
        execute_takes: SamePair<bool>,
        outer_bitmap_indices: u8,
    ) -> Self {
        Self {
            decode_deposit_amounts,
            execute_takes,
            outer_bitmap_count: outer_bitmap_indices,
            _marker: PhantomData,
        }
    }
}
