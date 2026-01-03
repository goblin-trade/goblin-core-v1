use crate::{
    goblin_error::GoblinError,
    input_processor::{Decodable, DecodeCtx},
    market::{CommonMarket, MarketAndKey, MarketVariant},
    state::{DynamicMarketHasher, MarketState},
    token::{CustomToken, DynamicIndex, TokenMarker},
};

#[derive(Clone, Copy, Default)]
pub struct Dynamic;

impl MarketVariant for Dynamic {
    const DISCRIMINATOR: u8 = 1;

    type TokenIndex = DynamicIndex;

    type DecodedMarket<B: TokenMarker, Q: TokenMarker> = MarketAndKey<Self, B, Q>;

    fn decode<'a, B, Q>(
        ctx: &'a DecodeCtx<'a>,
        custom_erc20_list: &[CustomToken],
    ) -> Result<Self::DecodedMarket<B, Q>, GoblinError>
    where
        B: TokenMarker,
        Q: TokenMarker,
        B::TokenIndex<Dynamic>: Decodable<'a>,
        Q::TokenIndex<Dynamic>: Decodable<'a>,
        MarketState<Dynamic, B, Q>: DynamicMarketHasher<B, Q>,
    {
        let common_market = CommonMarket::<Self, B, Q>::try_decode(ctx)?;

        let key =
            MarketState::<Dynamic, B, Q>::compute_slot_key(&common_market, custom_erc20_list)?;

        Ok(MarketAndKey {
            market: common_market,
            key,
        })
    }

    fn market_and_key_ref<'a, B, Q>(
        decoded_market: &'a Self::DecodedMarket<B, Q>,
    ) -> Result<&'a MarketAndKey<Self, B, Q>, GoblinError>
    where
        B: TokenMarker,
        Q: TokenMarker,
    {
        Ok(decoded_market)
    }
}
