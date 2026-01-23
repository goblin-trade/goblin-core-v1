use crate::{
    goblin_error::GoblinError,
    input_processor::{Decodable, DecodeCtx},
    market::{CommonMarket, MarketAndKey, MarketVariant},
    state::Preimage,
    token::{CustomERC20Data, TokenMarker},
};

#[derive(Clone, Copy, Default)]
pub struct Dynamic;

impl MarketVariant for Dynamic {
    const DISCRIMINATOR: u8 = 4;

    type DecodedMarket<B: TokenMarker, Q: TokenMarker> = MarketAndKey<Self, B, Q>;

    fn decode<B, Q>(
        ctx: &DecodeCtx,
        custom_erc20_list: &[CustomERC20Data],
    ) -> Result<Self::DecodedMarket<B, Q>, GoblinError>
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
