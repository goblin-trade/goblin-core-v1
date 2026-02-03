use crate::{
    goblin_error::GoblinError,
    input_processor::{Decodable, DecodeCtx},
    market::{Hardcoded, HardcodedMarketIndex, HardcodedMarkets, MarketAndKey, MarketLocator},
    token::TokenMarker,
};

impl<B, Q> MarketLocator<Hardcoded, B, Q> for HardcodedMarketIndex<B, Q>
where
    B: TokenMarker,
    Q: TokenMarker,
{
    fn decode_locator<'a>(ctx: &DecodeCtx, _erc20_list: ()) -> Result<Self, GoblinError> {
        HardcodedMarketIndex::<B, Q>::try_decode(ctx)
    }

    fn locate_market(&self) -> Result<&MarketAndKey<Hardcoded, B, Q>, GoblinError>
    where
        Self: HardcodedMarkets<B, Q>,
    {
        Self::HARDCODED_MARKETS
            .get(self.inner)
            .ok_or(GoblinError::InvalidHardcodedMarket)
    }
}
