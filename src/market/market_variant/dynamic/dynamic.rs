use crate::{
    goblin_error::GoblinError,
    input_processor::{ArgsBuffer, Decodable},
    market::{CommonMarket, MarketAndKey, MarketVariant},
    state::{DynamicMarketHasher, DynamicMarketKey},
    token::{CustomToken, DynamicIndex, TokenMarker},
};

#[derive(Clone, Copy, Default)]
pub struct Dynamic;

impl MarketVariant for Dynamic {
    const DISCRIMINATOR: u8 = 1;

    type TokenIndex = DynamicIndex;

    type DecodedMarket<B: TokenMarker, Q: TokenMarker> = MarketAndKey<Self, B, Q>;

    type MarketKey<B: TokenMarker, Q: TokenMarker> = DynamicMarketKey<B, Q>;

    fn decode<B, Q>(
        args: &ArgsBuffer,
        offset: &mut usize,
        len: usize,
        custom_erc20_list: &[CustomToken],
    ) -> Result<Self::DecodedMarket<B, Q>, GoblinError>
    where
        B: TokenMarker,
        Q: TokenMarker,
        B::TokenIndex<Dynamic>: Decodable,
        Q::TokenIndex<Dynamic>: Decodable,
        DynamicMarketKey<B, Q>: DynamicMarketHasher<B, Q>,
    {
        let common_market = CommonMarket::<Self, B, Q>::decode(args, offset, len)?;
        let key = DynamicMarketKey::hash(&common_market, custom_erc20_list)?;

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
