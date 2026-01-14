use crate::{
    goblin_error::GoblinError,
    input_processor::{Decodable, DecodeCtx},
    market::{CommonMarket, MarketAndKey, MarketVariant},
    state::Preimage,
    token::{CustomERC20Store, DynamicIndex, TokenMarker},
    types::Address,
};

#[derive(Clone, Copy, Default)]
pub struct Dynamic;

impl MarketVariant for Dynamic {
    const DISCRIMINATOR: u8 = 4;

    type TokenIndex = DynamicIndex;

    type DecodedMarket<B: TokenMarker, Q: TokenMarker> = MarketAndKey<Self, B, Q>;

    fn token_index_to_address(
        token_index: Self::TokenIndex,
        custom_erc20_list: &[CustomERC20Store],
    ) -> Result<Address, GoblinError> {
        token_index.address(custom_erc20_list)
    }

    fn decode<'a, B, Q>(
        ctx: &'a DecodeCtx<'a>,
        custom_erc20_list: &[CustomERC20Store],
    ) -> Result<Self::DecodedMarket<B, Q>, GoblinError>
    where
        B: TokenMarker,
        Q: TokenMarker,
        B::TokenIndex<Dynamic>: Decodable<'a>,
        Q::TokenIndex<Dynamic>: Decodable<'a>,
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
