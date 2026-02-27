use core::marker::PhantomData;

use crate::{
    axis::{
        leg::leg_matcher::LegMatcher, market::market_marker::MarketMarker,
        token::token_marker::TokenMarker,
    },
    matching::bitmap::row_column::RowColumn,
};

#[repr(C)]
pub struct InnerBitmap<M, B, Q>
where
    M: MarketMarker,
    B: TokenMarker,
    Q: TokenMarker,
{
    pub inner: [u8; 32],
    _marker: PhantomData<(M, B, Q)>,
}

impl<M, B, Q> InnerBitmap<M, B, Q>
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

    pub fn active<In>(&self, row_column: RowColumn<In>) -> bool
    where
        In: LegMatcher,
    {
        let row_bits = self.inner[row_column.row.inner as usize];
        let mask = 1u8 << row_column.column.inner;
        (row_bits & mask) != 0
    }
}
