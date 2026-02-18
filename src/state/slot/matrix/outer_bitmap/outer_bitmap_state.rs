use core::marker::PhantomData;

use crate::{
    axis::{market::market_marker::MarketMarker, token::token_marker::TokenMarker},
    state::outer_bitmap::{active_outer_bitmap::ActiveOuterBitmap, OuterBitmap},
};

const EMPTY_VALUE: [u8; 32] = [0; 32];
const CLOSED_SENTINEL: [u8; 32] = [0xFF; 32];

pub enum OuterBitmapState<M, B, Q>
where
    M: MarketMarker,
    B: TokenMarker,
    Q: TokenMarker,
{
    Empty(PhantomData<(M, B, Q)>),
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
        match value.inner {
            EMPTY_VALUE => OuterBitmapState::Empty(PhantomData),
            CLOSED_SENTINEL => OuterBitmapState::Closed(PhantomData),
            _ => OuterBitmapState::Active(ActiveOuterBitmap::new(value.inner)),
        }

        // if value.inner == CLOSED_SENTINEL {
        //     OuterBitmapState::Closed(PhantomData)
        // } else {
        //     OuterBitmapState::Active(ActiveOuterBitmap::new(value.inner))
        // }
    }
}
