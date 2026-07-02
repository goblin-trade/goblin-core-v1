use crate::{
    axis::{
        market::{market_locator::MarketLocator, CommonMarket, Dynamic, MarketReadables},
        token::{token_index::CustomERC20List, token_marker::TokenMarker},
    },
    goblin_error::GoblinError,
    input_processor::{Decodable, DecodeCtx},
    state::Preimage,
};

impl<B, Q> MarketLocator<Dynamic, B, Q> for MarketReadables<Dynamic, B, Q>
where
    B: TokenMarker,
    Q: TokenMarker,
{
    fn decode_locator<'a>(
        ctx: &DecodeCtx,
        erc20_list: CustomERC20List<'a>,
    ) -> Result<Self, GoblinError> {
        let common_market = CommonMarket::<Dynamic, B, Q>::try_decode(ctx)?;
        let preimage = common_market.get_preimage(erc20_list)?;
        let key = preimage.hash();

        Ok(MarketReadables {
            market: common_market,
            market_key: key,
        })
    }

    fn locate_market(&self) -> Result<&MarketReadables<Dynamic, B, Q>, GoblinError> {
        Ok(self)
    }
}
