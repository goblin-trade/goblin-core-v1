use crate::{
    axis::{
        market::{
            market_locator::{hardcoded::HardcodedMarkets, MarketLocator},
            market_marker::hardcoded::hardcoded_market_index::HardcodedMarketIndex,
            token_pair::TokenPair,
            Hardcoded, MarketReadables,
        },
        token::token_reader::TokenDataTriple,
    },
    goblin_error::GoblinError,
    input_processor::{Decodable, DecodeCtx},
};

impl<TP: TokenPair> MarketLocator<(Hardcoded, TP)> for HardcodedMarketIndex<TP> {
    fn decode_locator(
        ctx: &DecodeCtx,
        _token_data_triple: &TokenDataTriple,
    ) -> Result<Self, GoblinError> {
        HardcodedMarketIndex::<TP>::try_decode(ctx)
    }

    fn locate_market(&self) -> Result<&MarketReadables<(Hardcoded, TP)>, GoblinError>
    where
        Self: HardcodedMarkets<TP>,
    {
        Self::HARDCODED_MARKETS
            .get(self.inner)
            .ok_or(GoblinError::InvalidHardcodedMarket)
    }
}
