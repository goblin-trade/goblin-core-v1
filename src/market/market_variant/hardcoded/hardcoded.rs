use crate::{
    goblin_error::GoblinError,
    input_processor::{Decodable, DecodeCtx},
    market::{DangerousMarketIndex, HardcodedMarketList, MarketAndKey, MarketVariant},
    token::{CustomERC20Data, TokenMarker},
};

#[derive(Clone, Copy, Default)]
pub struct Hardcoded;

impl<B, Q> MarketVariant<B, Q> for Hardcoded
where
    B: TokenMarker,
    Q: TokenMarker,
{
    const DISCRIMINATOR: u8 = 3;

    type DecodedMarket = DangerousMarketIndex<B, Q>;

    fn decode(
        ctx: &DecodeCtx,
        _custom_erc20_list: &[CustomERC20Data],
    ) -> Result<Self::DecodedMarket, GoblinError>
    where
        DangerousMarketIndex<B, Q>: Decodable,
    {
        DangerousMarketIndex::<B, Q>::try_decode(ctx)
    }

    fn market_and_key_ref<'a>(
        decoded_market: &'a Self::DecodedMarket,
    ) -> Result<&'a MarketAndKey<Self, B, Q>, GoblinError>
    where
        B: TokenMarker,
        Q: TokenMarker,
        MarketAndKey<Hardcoded, B, Q>: HardcodedMarketList<B, Q>,
    {
        MarketAndKey::<Hardcoded, B, Q>::get_market(*decoded_market)
    }
}
