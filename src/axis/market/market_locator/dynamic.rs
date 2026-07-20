use crate::{
    axis::{
        market::{market_locator::MarketLocator, CommonMarket, Dynamic, MarketReadables},
        token::{token_marker::TokenMarker, token_reader::TokenDataTriple},
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
    fn decode_locator(
        ctx: &DecodeCtx,
        token_data_triple: &TokenDataTriple,
    ) -> Result<Self, GoblinError> {
        let common_market = CommonMarket::<Dynamic, B, Q>::try_decode(ctx)?;
        let preimage = common_market.get_preimage(token_data_triple)?;
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
