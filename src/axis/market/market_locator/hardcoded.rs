use crate::{
    axis::{
        market::{
            market_locator::MarketLocator,
            market_marker::hardcoded::{
                hardcoded_market_index::HardcodedMarketIndex, hardcoded_markets::HardcodedMarkets,
            },
            Hardcoded, MarketReadables,
        },
        token::{token_list::custom_erc20::CustomERC20List, token_marker::TokenMarker},
    },
    goblin_error::GoblinError,
    input_processor::{Decodable, DecodeCtx},
};

impl<B, Q> MarketLocator<Hardcoded, B, Q> for HardcodedMarketIndex<B, Q>
where
    B: TokenMarker,
    Q: TokenMarker,
{
    fn decode_locator<'a>(
        ctx: &DecodeCtx,
        _erc20_list: CustomERC20List<'a>,
    ) -> Result<Self, GoblinError> {
        HardcodedMarketIndex::<B, Q>::try_decode(ctx)
    }

    fn locate_market(&self) -> Result<&MarketReadables<Hardcoded, B, Q>, GoblinError>
    where
        Self: HardcodedMarkets<B, Q>,
    {
        Self::HARDCODED_MARKETS
            .get(self.inner)
            .ok_or(GoblinError::InvalidHardcodedMarket)
    }
}
