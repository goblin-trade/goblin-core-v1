use core::marker::PhantomData;

use crate::{
    axis::token::token_reader::TokenReader,
    goblin_error::GoblinError,
    input_processor::{Decodable, DecodeCtx},
};

/// Index to read a hardcoded market from the static list.
/// This index has NOT been validated for bounds. Bound check happens when reading
/// the market.
#[derive(Clone, Copy)]
pub struct HardcodedMarketIndex<B, Q>
where
    B: TokenReader,
    Q: TokenReader,
{
    pub inner: usize,
    _marker: PhantomData<(B, Q)>,
}

impl<B, Q> HardcodedMarketIndex<B, Q>
where
    B: TokenReader,
    Q: TokenReader,
{
    pub fn new(inner: usize) -> Self {
        Self {
            inner,
            _marker: PhantomData,
        }
    }
}

impl<B, Q> Decodable for HardcodedMarketIndex<B, Q>
where
    B: TokenReader,
    Q: TokenReader,
{
    fn try_decode(ctx: &DecodeCtx) -> Result<Self, GoblinError> {
        let market_index_raw = u8::try_decode(ctx)? as usize;
        Ok(HardcodedMarketIndex::<B, Q>::new(market_index_raw))
    }
}
