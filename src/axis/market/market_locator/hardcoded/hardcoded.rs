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

impl<TP> MarketLocator<(Hardcoded, TP)> for MarketIndex<(Hardcoded, TP)>
where
    TP: TokenPair,
    Self: HardcodedMarkets<TP>,
{
    fn decode_locator(
        ctx: &DecodeCtx,
        _token_data_triple: &TokenDataTriple,
    ) -> Result<Self, GoblinError> {
        MarketIndex::<(Hardcoded, TP)>::try_decode(ctx)
    }

    fn locate_market(&self) -> &MarketReadables<(Hardcoded, TP)> {
        &Self::HARDCODED_MARKETS[*self]
    }
}
