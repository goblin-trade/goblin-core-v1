use crate::{
    goblin_error::GoblinError,
    input_processor::{ArgsBuffer, Decodable},
    market::{DangerousMarketIndex, HardcodedMarketList, MarketAndKey, MarketVariant},
    state::HardcodedMarketKey,
    token::{CustomToken, HardcodedIndex, TokenMarker},
};

#[derive(Clone, Copy, Default)]
pub struct Hardcoded;

impl MarketVariant for Hardcoded {
    const DISCRIMINATOR: u8 = 0;

    type TokenIndex = HardcodedIndex;

    type DecodedMarket<B: TokenMarker, Q: TokenMarker> = DangerousMarketIndex<B, Q>;

    type MarketKey<B: TokenMarker, Q: TokenMarker> = HardcodedMarketKey<B, Q>;

    fn decode<B, Q>(
        args: &ArgsBuffer,
        offset: &mut usize,
        len: usize,
        _custom_erc20_list: &[CustomToken],
    ) -> Result<Self::DecodedMarket<B, Q>, GoblinError>
    where
        B: TokenMarker,
        Q: TokenMarker,
        DangerousMarketIndex<B, Q>: Decodable,
    {
        DangerousMarketIndex::<B, Q>::decode(args, offset, len)
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
