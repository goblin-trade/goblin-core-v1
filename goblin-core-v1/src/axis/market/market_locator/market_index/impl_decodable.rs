use crate::{
    axis::market::{
        market_locator::{hardcoded::HardcodedMarketList, MarketIndex},
        market_spec::MarketSpec,
    },
    goblin_error::GoblinError,
    input_processor::{Decodable, DecodeCtx},
    require,
};

impl<MS> Decodable for MarketIndex<MS>
where
    MS: MarketSpec,
    MS::Pair: HardcodedMarketList,
{
    fn try_decode(ctx: &DecodeCtx) -> Result<Self, GoblinError> {
        let market_index_raw = u8::try_decode(ctx)? as usize;
        let market_index = Self::new(market_index_raw);

        require!(
            market_index <= Self::MAX,
            GoblinError::InvalidHardcodedMarket
        );

        Ok(market_index)
    }
}
