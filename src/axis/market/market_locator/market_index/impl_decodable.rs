use crate::{
    axis::market::{
        market_locator::{hardcoded::HardcodedMarkets, MarketIndex},
        market_spec::MarketSpec,
    },
    goblin_error::GoblinError,
    input_processor::{Decodable, DecodeCtx},
    require,
};

impl<MS> Decodable for MarketIndex<MS>
where
    MS: MarketSpec,
    MarketIndex<MS>: HardcodedMarkets<MS::Pair>,
{
    fn try_decode(ctx: &DecodeCtx) -> Result<Self, GoblinError> {
        let market_index_raw = u8::try_decode(ctx)? as usize;

        require!(
            (market_index_raw as usize)
                < (<Self as HardcodedMarkets<MS::Pair>>::HARDCODED_MARKETS.len()),
            GoblinError::InvalidHardcodedMarket
        );

        Ok(Self::new(market_index_raw))
    }
}
