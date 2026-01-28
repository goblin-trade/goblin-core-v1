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

    type MarketLocator = DangerousMarketIndex<B, Q>;

    fn decode_locator(
        ctx: &DecodeCtx,
        _custom_erc20_list: &[CustomERC20Data],
    ) -> Result<Self::MarketLocator, GoblinError>
    where
        DangerousMarketIndex<B, Q>: Decodable,
    {
        DangerousMarketIndex::<B, Q>::try_decode(ctx)
    }

    fn locate_market<'a>(
        decoded_market: &'a Self::MarketLocator,
    ) -> Result<&'a MarketAndKey<Self, B, Q>, GoblinError>
    where
        B: TokenMarker,
        Q: TokenMarker,
        MarketAndKey<Hardcoded, B, Q>: HardcodedMarketList<B, Q>,
    {
        MarketAndKey::<Hardcoded, B, Q>::get_market(*decoded_market)
    }
}
