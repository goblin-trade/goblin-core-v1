use core::marker::PhantomData;

use crate::{
    axis::{
        leg::leg_matcher::LegMatcher, market::market_marker::MarketMarker,
        token::token_marker::TokenMarker,
    },
    matching::bitmap::outer_pos::OuterPos,
};

#[repr(C)]
#[derive(PartialEq)]
pub struct ActiveOuterBitmap<M, B, Q>
where
    M: MarketMarker,
    B: TokenMarker,
    Q: TokenMarker,
{
    pub inner: [u8; 32],
    _marker: PhantomData<(M, B, Q)>,
}

impl<M, B, Q> ActiveOuterBitmap<M, B, Q>
where
    M: MarketMarker,
    B: TokenMarker,
    Q: TokenMarker,
{
    pub const fn new(inner: [u8; 32]) -> Self {
        Self {
            inner,
            _marker: PhantomData,
        }
    }

    pub fn active<In>(&self, outer_pos: OuterPos<In>) -> bool
    where
        In: LegMatcher,
    {
        let idx = outer_pos.inner as usize;

        let byte_index = idx / 8;
        let bit_index = idx % 8;

        let byte = self.inner[byte_index];
        (byte >> bit_index) & 1 == 1
    }
}
