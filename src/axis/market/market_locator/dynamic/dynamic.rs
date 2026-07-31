use crate::{
    axis::{
        market::{
            market_locator::{hardcoded::HardcodedMarkets, MarketLocator},
            token_pair::TokenPair,
            CommonMarket, Dynamic, MarketReadables,
        },
        token::token_reader::TokenDataTriple,
    },
    goblin_error::GoblinError,
    input_processor::{Decodable, DecodeCtx},
    state::Preimage,
};

impl<TP> MarketLocator<TP> for Dynamic
where
    TP: TokenPair + HardcodedMarkets<TP>,
{
    type Locator = MarketReadables<(Self, TP)>;

    fn decode_locator(
        ctx: &DecodeCtx,
        token_data_triple: &TokenDataTriple,
    ) -> Result<Self::Locator, GoblinError> {
        let common_market = CommonMarket::<(Dynamic, TP)>::try_decode(ctx)?;
        let preimage = common_market.get_preimage(token_data_triple)?;
        let key = preimage.hash();

        Ok(MarketReadables {
            market: common_market,
            market_key: key,
        })
    }

    fn locate_market(locator: &Self::Locator) -> &MarketReadables<(Dynamic, TP)> {
        locator
    }
}
