use crate::{
    axis::market::token_pair::TokenPair,
    goblin_error::GoblinError,
    input_processor::{Decodable, DecodeCtx},
};
use core::marker::PhantomData;

/// Index to read a hardcoded market from the static list.
/// This index has NOT been validated for bounds. Bound check happens when reading
/// the market.
#[derive(Clone, Copy)]
pub struct HardcodedMarketIndex<TP: TokenPair> {
    pub inner: usize,
    _marker: PhantomData<TP>,
}

impl<TP: TokenPair> HardcodedMarketIndex<TP> {
    pub fn new(inner: usize) -> Self {
        Self {
            inner,
            _marker: PhantomData,
        }
    }
}

impl<TP: TokenPair> Decodable for HardcodedMarketIndex<TP> {
    fn try_decode(ctx: &DecodeCtx) -> Result<Self, GoblinError> {
        let market_index_raw = u8::try_decode(ctx)? as usize;
        Ok(HardcodedMarketIndex::new(market_index_raw))
    }
}
