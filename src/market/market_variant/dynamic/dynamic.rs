use crate::{
    goblin_error::GoblinError,
    input_processor::{Decodable, DecodeCtx},
    market::{CommonMarket, MarketAndKey, MarketVariant},
    state::Preimage,
    token::{CustomERC20Data, TokenMarker},
};

#[derive(Clone, Copy, Default)]
pub struct Dynamic;

impl<B, Q> MarketVariant<B, Q> for Dynamic
where
    B: TokenMarker,
    Q: TokenMarker,
{
    const DISCRIMINATOR: u8 = 4;

    type MarketLocator = MarketAndKey<Self, B, Q>;

    fn decode_locator(
        ctx: &DecodeCtx,
        custom_erc20_list: &[CustomERC20Data],
    ) -> Result<Self::MarketLocator, GoblinError>
    where
        B: TokenMarker,
        Q: TokenMarker,
    {
        let common_market = CommonMarket::<Self, B, Q>::try_decode(ctx)?;
        let preimage = common_market.get_preimage(custom_erc20_list)?;
        let key = preimage.hash();

        Ok(MarketAndKey {
            market: common_market,
            key,
        })
    }

    fn locate_market<'a>(
        decoded_market: &'a Self::MarketLocator,
    ) -> Result<&'a MarketAndKey<Self, B, Q>, GoblinError>
    where
        B: TokenMarker,
        Q: TokenMarker,
    {
        Ok(decoded_market)
    }
}
