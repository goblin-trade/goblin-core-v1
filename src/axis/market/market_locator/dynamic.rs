use crate::{
    axis::{
        market::{market_locator::MarketLocator, CommonMarket, Dynamic, MarketAndKey},
        token::token_marker::{custom_erc20::custom_erc20_data::CustomERC20Data, TokenMarker},
    },
    goblin_error::GoblinError,
    input_processor::{Decodable, DecodeCtx},
    state::Preimage,
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
            market_key: key,
        })
    }

    fn locate_market(&self) -> Result<&MarketAndKey<Dynamic, B, Q>, GoblinError> {
        Ok(self)
    }
}
