use crate::{
    goblin_error::GoblinError,
    input_processor::{Decodable, DecodeCtx},
    market::{DangerousMarketIndex, HardcodedMarketList, MarketAndKey, MarketVariant},
    token::{CustomToken, HardcodedIndex, TokenMarker},
};

#[derive(Clone, Copy, Default)]
pub struct Hardcoded;

impl MarketVariant for Hardcoded {
    const DISCRIMINATOR: u8 = 0;

    type TokenIndex = HardcodedIndex;

    type DecodedMarket<B: TokenMarker, Q: TokenMarker> = DangerousMarketIndex<B, Q>;

    fn decode<'a, B, Q>(
        ctx: &'a DecodeCtx<'a>,
        _custom_erc20_list: &[CustomToken],
    ) -> Result<Self::DecodedMarket<B, Q>, GoblinError>
    where
        B: TokenMarker,
        Q: TokenMarker,
        DangerousMarketIndex<B, Q>: Decodable<'a>,
    {
        DangerousMarketIndex::<B, Q>::try_decode(ctx)
    }

    fn market_and_key_ref<'a, B, Q>(
        decoded_market: &'a Self::DecodedMarket<B, Q>,
    ) -> Result<&'a MarketAndKey<Self, B, Q>, GoblinError>
    where
        B: TokenMarker + 'static,
        Q: TokenMarker + 'static,
        MarketAndKey<Hardcoded, B, Q>: HardcodedMarketList<B, Q>,
    {
        MarketAndKey::<Hardcoded, B, Q>::get_market(*decoded_market)
    }
}
