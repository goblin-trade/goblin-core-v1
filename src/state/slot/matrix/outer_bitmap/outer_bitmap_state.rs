use core::marker::PhantomData;

use crate::{
    axis::{market::market_marker::MarketMarker, token::token_marker::TokenMarker},
    state::outer_bitmap::{active_outer_bitmap::ActiveOuterBitmap, OuterBitmap},
};

pub const CLOSED_SENTINEL: [u8; 32] = [0xFF; 32];

pub enum OuterBitmapState<M, B, Q>
where
    M: MarketMarker,
    B: TokenMarker,
    Q: TokenMarker,
{
    Closed(PhantomData<(M, B, Q)>),
    Active(ActiveOuterBitmap<M, B, Q>),
}

impl<M, B, Q> From<OuterBitmap<M, B, Q>> for OuterBitmapState<M, B, Q>
where
    M: MarketMarker,
    B: TokenMarker,
    Q: TokenMarker,
{
    fn from(value: OuterBitmap<M, B, Q>) -> Self {
        if value.inner == CLOSED_SENTINEL {
            OuterBitmapState::Closed(PhantomData)
        } else {
            OuterBitmapState::Active(ActiveOuterBitmap::new(value.inner))
        }
    }
}
