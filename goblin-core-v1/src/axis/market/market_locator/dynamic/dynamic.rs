use crate::{
    axis::{
        market::{
            market_locator::{hardcoded::HardcodedMarketList, MarketLocator},
            token_pair::TokenPair,
            CommonMarket, Dynamic, MarketReadables,
        },
        token::token_reader::TokenDataTriple,
    },
    goblin_error::GoblinError,
    input_processor::{DecodeCtx, FixedDecode},
    state::Preimage,
};

impl<TP> MarketLocator<TP> for Dynamic
where
    TP: TokenPair + HardcodedMarketList,
{
    type Locator = MarketReadables<(Self, TP)>;

    fn decode_locator(
        ctx: &DecodeCtx,
        token_data_triple: &TokenDataTriple,
    ) -> Result<Self::Locator, GoblinError> {
        let common_market = CommonMarket::<(Dynamic, TP)>::try_fixed_decode(ctx)?;
        common_market.lot_size_pair.validate()?;

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
