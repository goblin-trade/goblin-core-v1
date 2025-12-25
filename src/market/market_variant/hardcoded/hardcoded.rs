use crate::{
    goblin_error::GoblinError,
    input_processor::{ArgsBuffer, Decodable},
    market::{HardcodedMarketIndex, HardcodedMarketList, MarketAndKey, MarketVariant},
    state::HardcodedMarketKey,
    token::{CustomToken, HardcodedIndex, TokenMarker},
};

#[derive(Clone, Copy, Default)]
pub struct Hardcoded;

impl MarketVariant for Hardcoded {
    const DISCRIMINATOR: u8 = 0;

    type TokenIndex = HardcodedIndex;

    type DecodedMarket<B: TokenMarker, Q: TokenMarker> = HardcodedMarketIndex;

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
    {
        <HardcodedMarketIndex as Decodable<HardcodedMarketIndex>>::decode(args, offset, len)
    }

    fn market_and_key_ref<'a, B, Q>(
        decoded_market: &'a Self::DecodedMarket<B, Q>,
    ) -> Result<&'a MarketAndKey<Self, B, Q>, GoblinError>
    where
        B: TokenMarker + 'static,
        Q: TokenMarker + 'static,
        MarketAndKey<Hardcoded, B, Q>: HardcodedMarketList<B, Q>,
    {
        MarketAndKey::<Hardcoded, B, Q>::HARDCODED_MARKET_LIST
            .get(decoded_market.0)
            .ok_or(GoblinError::InvalidHardcodedMarket)
    }
}
