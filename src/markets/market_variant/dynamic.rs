use crate::{
    goblin_error::GoblinError,
    markets::{
        CommonMarket, Dynamic, DynamicMarket, MarketVariant, MarketWithKey, MarketWithKeyRef,
    },
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

    type BlackBox<B: TokenMarker, Q: TokenMarker> = MarketWithKey<Self, B, Q>;

    type MarketKey<B: TokenMarker, Q: TokenMarker> = DynamicMarketKey<B, Q>;

    type Market<B: TokenMarker, Q: TokenMarker> = DynamicMarket<B, Q>;

    fn get_market_with_key_ref<'a, B, Q>(
        black_box: &'a Self::BlackBox<B, Q>,
    ) -> Result<MarketWithKeyRef<'a, Self, B, Q>, GoblinError>
    where
        B: TokenMarker,
        Q: TokenMarker,
    {
        Ok(MarketWithKeyRef {
            common_market: &black_box.common_market,
            key: &black_box.key,
        })
    }

    fn get_market_key<B, Q>(
        market: &Self::Market<B, Q>,
        custom_erc20_list: &[CustomToken],
    ) -> Result<Self::MarketKey<B, Q>, GoblinError>
    where
        B: TokenMarker,
        Q: TokenMarker,
        DynamicMarketKey<B, Q>: DynamicMarketHasher<B, Q>,
    {
        DynamicMarketKey::hash(&market.common, custom_erc20_list)
    }

    fn common_market<B, Q>(market: &Self::Market<B, Q>) -> &CommonMarket<Self, B, Q>
    where
        B: TokenMarker,
        Q: TokenMarker,
    {
        &market.common
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
