use crate::{
    goblin_error::GoblinError,
    input_processor::{Decodable, DecodeCtx},
    market::{DangerousMarketIndex, HardcodedMarketList, MarketAndKey, MarketVariant},
    token::{CustomERC20Store, HardcodedERC20, TokenIndex, TokenMarker},
    types::Address,
};

#[derive(Clone, Copy, Default)]
pub struct Hardcoded;

impl MarketVariant for Hardcoded {
    const DISCRIMINATOR: u8 = 3;

    type TokenIndex = TokenIndex<HardcodedERC20>;

    type DecodedMarket<B: TokenMarker, Q: TokenMarker> = DangerousMarketIndex<B, Q>;

    // fn token_index_to_address(
    //     token_index: Self::TokenIndex,
    //     _custom_erc20_list: &[CustomERC20Store],
    // ) -> Result<Address, GoblinError> {
    //     // TODO simplify when we use dedicated marker types for Hardcoded and Custom
    //     Ok(token_index.get_token().address)
    // }

    fn decode<'a, B, Q>(
        ctx: &'a DecodeCtx<'a>,
        _custom_erc20_list: &[CustomERC20Store],
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
