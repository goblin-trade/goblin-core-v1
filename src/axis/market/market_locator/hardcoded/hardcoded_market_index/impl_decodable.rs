use crate::{
    axis::market::{market_locator::hardcoded::HardcodedMarketIndex, TokenPair},
    goblin_error::GoblinError,
    input_processor::{Decodable, DecodeCtx},
};

impl<TP: TokenPair> Decodable for HardcodedMarketIndex<TP> {
    fn try_decode(ctx: &DecodeCtx) -> Result<Self, GoblinError> {
        let market_index_raw = u8::try_decode(ctx)? as usize;
        Ok(HardcodedMarketIndex::new(market_index_raw))
    }
}
