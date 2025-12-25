use crate::{
    goblin_error::GoblinError,
    input_processor::{ArgsBuffer, Decodable},
    markets::{CommonMarket, Dynamic, MarketAndKey, MarketVariant},
    settlement::global_delta::{
        ERC20Delta, ERC20DeltaList, ERC20MakerDeltaKey, ERC20MakerDeltas, ERC20SenderDeltas,
        UnsidedMakerDelta,
    },
    state::{DynamicMarketHasher, DynamicMarketKey},
    token::{CustomToken, DynamicIndex, HardcodedToken, TokenMarker},
    types::{Address, TupleReader},
};

impl MarketVariant for Dynamic {
    const DISCRIMINATOR: u8 = 1;

    type TokenIndex = DynamicIndex;

    type DecodedMarket<B: TokenMarker, Q: TokenMarker> = MarketAndKey<Self, B, Q>;

    type MarketKey<B: TokenMarker, Q: TokenMarker> = DynamicMarketKey<B, Q>;

    fn decode<B, Q>(
        args: &ArgsBuffer,
        offset: &mut usize,
        len: usize,
        custom_erc20_list: &[CustomToken],
    ) -> Result<Self::DecodedMarket<B, Q>, GoblinError>
    where
        B: TokenMarker + Decodable<B::TokenIndex<Dynamic>>,
        Q: TokenMarker + Decodable<Q::TokenIndex<Dynamic>>,
        DynamicMarketKey<B, Q>: DynamicMarketHasher<B, Q>,
        CommonMarket<Dynamic, B, Q>: Decodable<CommonMarket<Dynamic, B, Q>>,
    {
        let common_market = CommonMarket::<Self, B, Q>::decode(args, offset, len)?;
        let key = DynamicMarketKey::hash(&common_market, custom_erc20_list)?;

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

    fn token_sender_delta_mut(
        token_index: Self::TokenIndex,
        token_sender_deltas: &mut ERC20SenderDeltas,
    ) -> &mut ERC20Delta {
        match token_index {
            DynamicIndex::Hardcoded(hardcoded_token_index) => {
                HardcodedToken::get_leg_mut(token_sender_deltas)
                    .get_delta_mut(hardcoded_token_index)
            }

            DynamicIndex::Custom(custom_token_index) => {
                CustomToken::get_leg_mut(token_sender_deltas).get_delta_mut(custom_token_index)
            }
        }
    }

    fn token_maker_delta_mut(
        token_index: Self::TokenIndex,
        maker: Address,
        token_maker_deltas: &mut ERC20MakerDeltas,
    ) -> Option<&mut UnsidedMakerDelta> {
        match token_index {
            DynamicIndex::Hardcoded(hardcoded_token_index) => {
                let key = ERC20MakerDeltaKey {
                    maker,
                    token_index: hardcoded_token_index,
                };

                HardcodedToken::get_leg_mut(token_maker_deltas).get_or_insert_mut(key)
            }

            DynamicIndex::Custom(custom_token_index) => {
                let key = ERC20MakerDeltaKey {
                    maker,
                    token_index: custom_token_index,
                };

                CustomToken::get_leg_mut(token_maker_deltas).get_or_insert_mut(key)
            }
        }
    }
}
