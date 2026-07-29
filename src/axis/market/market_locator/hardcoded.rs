use crate::{
    axis::{
        leg::Pair,
        market::{
            market_locator::MarketLocator,
            market_marker::hardcoded::{
                hardcoded_market_index::HardcodedMarketIndex, hardcoded_markets::HardcodedMarkets,
            },
            Hardcoded, MarketReadables,
        },
        token::{token_marker::TokenMarker, token_reader::TokenDataTriple},
    },
    goblin_error::GoblinError,
    input_processor::{Decodable, DecodeCtx},
};

impl<B, Q> MarketLocator<(Hardcoded, Pair<B, Q>)> for HardcodedMarketIndex<B, Q>
where
    B: TokenMarker,
    Q: TokenMarker,
{
    fn decode_locator(
        ctx: &DecodeCtx,
        _token_data_triple: &TokenDataTriple,
    ) -> Result<Self, GoblinError> {
        HardcodedMarketIndex::<B, Q>::try_decode(ctx)
    }

    fn locate_market(&self) -> Result<&MarketReadables<(Hardcoded, Pair<B, Q>)>, GoblinError>
    where
        Self: HardcodedMarkets<B, Q>,
    {
        Self::HARDCODED_MARKETS
            .get(self.inner)
            .ok_or(GoblinError::InvalidHardcodedMarket)
    }
}
