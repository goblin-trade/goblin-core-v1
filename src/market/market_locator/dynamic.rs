use crate::{
    goblin_error::GoblinError,
    input_processor::{Decodable, DecodeCtx},
    market::{CommonMarket, Dynamic, Hardcoded, HardcodedMarketList, MarketAndKey, MarketLocator},
    state::Preimage,
    token::{CustomERC20Data, TokenMarker},
};

impl<B, Q> MarketLocator<Dynamic, B, Q> for MarketAndKey<Dynamic, B, Q>
where
    B: TokenMarker,
    Q: TokenMarker,
{
    fn decode_locator(
        ctx: &DecodeCtx,
        custom_erc20_list: &[CustomERC20Data],
    ) -> Result<Self, GoblinError> {
        let common_market = CommonMarket::<Dynamic, B, Q>::try_decode(ctx)?;
        let preimage = common_market.get_preimage(custom_erc20_list)?;
        let key = preimage.hash();

        Ok(MarketAndKey {
            market: common_market,
            key,
        })
    }

    fn locate_market<'a>(
        decoded_market: &'a Self,
    ) -> Result<&'a MarketAndKey<Dynamic, B, Q>, GoblinError>
    where
        MarketAndKey<Hardcoded, B, Q>: HardcodedMarketList<B, Q>,
    {
        Ok(decoded_market)
    }
}
