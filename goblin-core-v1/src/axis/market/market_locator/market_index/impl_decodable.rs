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

        require!(
            (market_index_raw as usize)
                < (<MS::Pair as HardcodedMarketList>::HARDCODED_MARKET_LIST.len()),
            GoblinError::InvalidHardcodedMarket
        );

        Ok(Self::new(market_index_raw))
    }
}
