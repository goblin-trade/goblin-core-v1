use crate::{
    markets::PairShape,
    settlement::global_delta::{
        ERC20Delta, ERC20DeltaList, ERC20MakerDeltaKey, ERC20MakerDeltas, ERC20SenderDeltas,
        UnsidedMakerDelta,
    },
    state::{DynamicMarketKey, HardcodedMarketKey, SlotKey},
    token::{CustomToken, DynamicIndex, HardcodedIndex, HardcodedToken, TokenMarker},
    types::{Address, TupleReader},
};

/// We have 2 market variants
///
/// * Hardcoded market- has hardcoded tokens
/// * Dynamic market- has dynamic tokens that can be either dynamic or custom
pub trait MarketVariant: Clone + Copy {
    const DISCRIMINATOR: u8;

    /// Key to read market state slot
    type MarketKey<B: TokenMarker, Q: TokenMarker>: SlotKey;

    // Both funtions and structs are symmetric. Can we write an abstraction?
    // Maybe I can have a common accessor trait shared by both
    //
    // Or something like Pair<>
    // ERC20Pair<H, C> { hardcoded: H, custom: C }
    fn token_sender_delta_mut(self, token_sender_deltas: &mut ERC20SenderDeltas)
        -> &mut ERC20Delta;

    fn token_maker_delta_mut(
        self,
        maker: Address,
        token_maker_deltas: &mut ERC20MakerDeltas,
    ) -> Option<&mut UnsidedMakerDelta>;
}

impl MarketVariant for HardcodedIndex {
    const DISCRIMINATOR: u8 = 0;

    type MarketKey<B: TokenMarker, Q: TokenMarker> = HardcodedMarketKey<B, Q>;

    fn token_sender_delta_mut(
        self,
        token_sender_deltas: &mut ERC20SenderDeltas,
    ) -> &mut ERC20Delta {
        HardcodedToken::get_leg_mut(token_sender_deltas).get_delta_mut(self)
    }

    fn token_maker_delta_mut(
        self,
        maker: Address,
        token_maker_deltas: &mut ERC20MakerDeltas,
    ) -> Option<&mut UnsidedMakerDelta> {
        let key = ERC20MakerDeltaKey {
            maker,
            token_index: self,
        };

        HardcodedToken::get_leg_mut(token_maker_deltas).get_or_insert_mut(key)
    }
}

impl MarketVariant for DynamicIndex {
    const DISCRIMINATOR: u8 = 1;

    type MarketKey<B: TokenMarker, Q: TokenMarker> = DynamicMarketKey<B, Q>;

    fn token_sender_delta_mut(
        self,
        token_sender_deltas: &mut ERC20SenderDeltas,
    ) -> &mut ERC20Delta {
        match self {
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
        self,
        maker: Address,
        token_maker_deltas: &mut ERC20MakerDeltas,
    ) -> Option<&mut UnsidedMakerDelta> {
        match self {
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
