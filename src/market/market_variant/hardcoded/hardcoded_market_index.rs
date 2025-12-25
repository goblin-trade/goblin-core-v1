use core::marker::PhantomData;

use crate::{
    goblin_error::GoblinError,
    input_processor::{ArgsBuffer, ArgsDecoder, Decodable},
    token::TokenMarker,
};

pub struct HardcodedMarketIndex<B: TokenMarker, Q: TokenMarker> {
    pub inner: usize,
    _marker: PhantomData<(B, Q)>,
}

impl<B: TokenMarker, Q: TokenMarker> HardcodedMarketIndex<B, Q> {
    pub fn new(inner: usize) -> Self {
        Self {
            inner,
            _marker: PhantomData,
        }
    }
}

impl<B: TokenMarker, Q: TokenMarker> Decodable<HardcodedMarketIndex<B, Q>>
    for HardcodedMarketIndex<B, Q>
{
    fn decode(
        args: &ArgsBuffer,
        offset: &mut usize,
        len: usize,
    ) -> Result<HardcodedMarketIndex<B, Q>, GoblinError> {
        let market_index_raw = args.decode::<u8>(offset, len)? as usize;
        Ok(HardcodedMarketIndex::<B, Q>::new(market_index_raw))
    }
}
