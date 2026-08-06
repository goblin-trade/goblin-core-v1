use crate::{
    axis::{
        market::{
            market_locator::{hardcoded::HardcodedMarketList, MarketIndex, MarketLocator},
            token_pair::TokenPair,
            Hardcoded, MarketReadables,
        },
        token::token_reader::TokenDataTriple,
    },
    goblin_error::GoblinError,
    input_processor::{DecodeCtx, FixedDecode},
};

impl<TP> MarketLocator<TP> for Hardcoded
where
    TP: TokenPair + HardcodedMarketList,
{
    type Locator = MarketIndex<(Self, TP)>;

    fn decode_locator(
        ctx: &DecodeCtx,
        _token_data_triple: &TokenDataTriple,
    ) -> Result<Self::Locator, GoblinError> {
        MarketIndex::try_fixed_decode(ctx)
    }

    fn locate_market(locator: &Self::Locator) -> &MarketReadables<(Hardcoded, TP)> {
        &TP::HARDCODED_MARKET_LIST[*locator]
    }
}
