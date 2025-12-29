use core::marker::PhantomData;

use crate::{
    goblin_error::GoblinError,
    input_processor::{Decodable, DecodeCtx},
    token::TokenMarker,
};

/// Index to read a hardcoded market from the static list.
/// This index has NOT been validated for bounds. Bound check happens when reading
/// the market.
#[derive(Clone, Copy)]
pub struct DangerousMarketIndex<B, Q>
where
    B: TokenMarker,
    Q: TokenMarker,
{
    pub inner: usize,
    _marker: PhantomData<(B, Q)>,
}

impl<B, Q> DangerousMarketIndex<B, Q>
where
    B: TokenMarker,
    Q: TokenMarker,
{
    pub fn new(inner: usize) -> Self {
        Self {
            inner,
            _marker: PhantomData,
        }
    }
}

impl<'a, B, Q> Decodable<'a> for DangerousMarketIndex<B, Q>
where
    B: TokenMarker,
    Q: TokenMarker,
{
    fn decode(ctx: &DecodeCtx<'a>) -> Result<Self, GoblinError> {
        let market_index_raw = ctx.decode::<u8>()? as usize;
        Ok(DangerousMarketIndex::<B, Q>::new(market_index_raw))
    }
}
