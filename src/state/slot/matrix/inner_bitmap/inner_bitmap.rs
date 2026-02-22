use core::marker::PhantomData;

use crate::{
    axis::{
        leg::leg_matcher::LegMatcher, market::market_marker::MarketMarker,
        token::token_marker::TokenMarker,
    },
    matching::bitmap::{
        inner_coordinates::{self, InnerCoordinates},
        row::Row,
    },
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

    pub fn active<In>(&self, row: Row<In>) -> bool
    where
        In: LegMatcher,
    {
        self.inner[row.inner as usize] != 0
    }

    pub fn active_v2<In>(&self, inner_coordinates: InnerCoordinates<In>) -> bool
    where
        In: LegMatcher,
    {
        let row_bits = self.inner[inner_coordinates.row.inner as usize];
        let mask = 1u8 << inner_coordinates.column.inner;
        (row_bits & mask) != 0
    }
}
