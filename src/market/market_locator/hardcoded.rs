use crate::{
    goblin_error::GoblinError,
    input_processor::{Decodable, DecodeCtx},
    market::{Hardcoded, HardcodedMarketIndex, HardcodedMarketList, MarketAndKey, MarketLocator},
    token::{CustomERC20Data, TokenMarker},
};

impl<B, Q> MarketLocator<Hardcoded, B, Q> for HardcodedMarketIndex<B, Q>
where
    B: TokenMarker,
    Q: TokenMarker,
{
    fn decode_locator(
        ctx: &DecodeCtx,
        _custom_erc20_list: &[CustomERC20Data],
    ) -> Result<Self, GoblinError> {
        HardcodedMarketIndex::<B, Q>::try_decode(ctx)
    }

    fn locate_market(&self) -> Result<&MarketAndKey<Hardcoded, B, Q>, GoblinError>
    where
        Self: HardcodedMarketList<B, Q>,
    {
        Self::HARDCODED_MARKET_LIST
            .get(self.inner)
            .ok_or(GoblinError::InvalidHardcodedMarket)
    }
}
