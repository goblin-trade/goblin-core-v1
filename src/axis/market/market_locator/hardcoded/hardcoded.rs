use crate::{
    axis::{
        market::{
            market_locator::{hardcoded::HardcodedMarkets, MarketIndex, MarketLocator},
            token_pair::TokenPair,
            Hardcoded, MarketReadables,
        },
        token::token_reader::TokenDataTriple,
    },
    goblin_error::GoblinError,
    input_processor::{Decodable, DecodeCtx},
};

impl<TP> MarketLocator<TP> for Hardcoded
where
    TP: TokenPair,
    TP: HardcodedMarkets<TP>,
{
    type Locator = MarketIndex<(Self, TP)>;

    fn decode_locator(
        ctx: &DecodeCtx,
        _token_data_triple: &TokenDataTriple,
    ) -> Result<Self::Locator, GoblinError> {
        MarketIndex::<(Hardcoded, TP)>::try_decode(ctx)
    }

    fn locate_market(locator: &Self::Locator) -> &MarketReadables<(Hardcoded, TP)> {
        &TP::HARDCODED_MARKETS[*locator]
    }
}
