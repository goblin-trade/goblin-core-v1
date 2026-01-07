use crate::{
    goblin_error::GoblinError,
    input_processor::{Decodable, DecodeCtx},
    market::{CommonMarket, MarketAndKey, MarketVariant},
    state::{MarketPreimage, MarketState, Preimage},
    token::{CustomToken, DynamicIndex, TokenMarker},
    types::{Address, Base, Pair, Quote, Tuple, TupleReader},
};

#[derive(Clone, Copy, Default)]
pub struct Dynamic;

impl MarketVariant for Dynamic {
    const DISCRIMINATOR: u8 = 4;

    type TokenIndex = DynamicIndex;

    type DecodedMarket<B: TokenMarker, Q: TokenMarker> = MarketAndKey<Self, B, Q>;

    fn token_index_to_address_inner(
        token_index: Self::TokenIndex,
        custom_erc20_list: &[CustomToken],
    ) -> Result<Address, GoblinError> {
        token_index.address(custom_erc20_list)
    }

    fn decode<'a, B, Q>(
        ctx: &'a DecodeCtx<'a>,
        custom_erc20_list: &[CustomToken],
    ) -> Result<Self::DecodedMarket<B, Q>, GoblinError>
    where
        B: TokenMarker,
        Q: TokenMarker,
        B::TokenIndex<Dynamic>: Decodable<'a>,
        Q::TokenIndex<Dynamic>: Decodable<'a>,
    {
        let common_market = CommonMarket::<Self, B, Q>::try_decode(ctx)?;

        // TODO util for conversion
        let base_token_index = Base::get(&common_market.token_index_pair);
        let quote_token_index = Quote::get(&common_market.token_index_pair);
        let base_token_address =
            B::token_index_to_address_outer(base_token_index, custom_erc20_list)?;
        let quote_token_address =
            Q::token_index_to_address_outer(quote_token_index, custom_erc20_list)?;

        let preimage = MarketPreimage::<Dynamic, B, Q>::new(
            common_market.lot_size_pair,
            common_market.tick_size,
            Tuple::new(base_token_address, quote_token_address),
        );

        let key = preimage.generate();

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
