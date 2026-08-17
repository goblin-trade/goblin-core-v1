use crate::{
    axis::market::market_locator::{hardcoded::HardcodedMarketList, MarketIndex},
    axis_helpers::MarketSpec,
    goblin_error::GoblinError,
    input_processor::{DecodeCtx, FixedDecode},
    require,
};

impl<'a, MS> FixedDecode<'a> for MarketIndex<MS>
where
    MS: MarketSpec,
    MS::Pair: HardcodedMarketList,
{
    const ENCODED_SIZE: usize = 1;

    fn raw_fixed_decode(ctx: &'a DecodeCtx) -> Self {
        let market_index_raw = u8::raw_fixed_decode(ctx) as usize;
        Self::new(market_index_raw)
    }

    fn validate(&self) -> Result<(), GoblinError> {
        require!(*self <= Self::MAX, GoblinError::InvalidPayload);
        Ok(())
    }
}
