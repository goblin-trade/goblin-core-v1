use crate::{
    goblin_error::GoblinError,
    input_processor::{Decodable, DecodeCtx},
    market::{CommonMarket, MarketAndKey, MarketLocator},
    state::Preimage,
    token::{CustomERC20Data, TokenMarker},
    types::Dynamic,
};

impl<B, Q> MarketLocator<Dynamic, B, Q> for MarketAndKey<Dynamic, B, Q>
where
    B: TokenMarker,
    Q: TokenMarker,
{
    fn decode_locator<'a>(
        ctx: &DecodeCtx,
        erc20_list: &'a [CustomERC20Data],
    ) -> Result<Self, GoblinError> {
        let common_market = CommonMarket::<Dynamic, B, Q>::try_decode(ctx)?;
        let preimage = common_market.get_preimage(erc20_list)?;
        let key = preimage.hash();

        Ok(MarketAndKey {
            market: common_market,
            key,
        })
    }

    fn locate_market(&self) -> Result<&MarketAndKey<Dynamic, B, Q>, GoblinError> {
        Ok(self)
    }
}
