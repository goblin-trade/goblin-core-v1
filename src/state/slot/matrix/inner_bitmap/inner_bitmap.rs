use core::marker::PhantomData;

use crate::axis::{market::market_marker::MarketMarker, token::token_marker::TokenMarker};

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
}
