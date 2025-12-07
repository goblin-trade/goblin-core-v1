use crate::{
    markets::PairShape,
    settlement::global_delta::{
        ERC20Delta, ERC20DeltaList, TokenMakerDeltaKey, TokenMakerDeltas, TokenSenderDeltas,
        UnsidedMakerDelta,
    },
    state::{DynamicMarketKey, HardcodedMarketKey, SlotKey},
    token::{DynamicIndex, HardcodedIndex},
    types::Address,
};

/// We have 2 market variants
///
/// * Hardcoded market- has hardcoded tokens
/// * Dynamic market- has dynamic tokens that can be either dynamic or custom
pub trait MarketVariant {
    const DISCRIMINATOR: u8;

    /// Key to read market state slot
    type MarketKey<P: PairShape>: SlotKey;

    // Both funtions and structs are symmetric. Can we write an abstraction?
    // Maybe I can have a common accessor trait shared by both
    //
    // Or something like Pair<>
    // ERC20Pair<H, C> { hardcoded: H, custom: C }
    fn token_sender_delta_mut(self, token_sender_deltas: &mut TokenSenderDeltas)
        -> &mut ERC20Delta;

    fn token_maker_delta_mut(
        self,
        maker: Address,
        token_maker_deltas: &mut TokenMakerDeltas,
    ) -> Option<&mut UnsidedMakerDelta>;
}

impl MarketVariant for HardcodedIndex {
    const DISCRIMINATOR: u8 = 0;

    type MarketKey<P: PairShape> = HardcodedMarketKey<P>;

    fn token_sender_delta_mut(
        self,
        token_sender_deltas: &mut TokenSenderDeltas,
    ) -> &mut ERC20Delta {
        token_sender_deltas
            .hardcoded_token_deltas
            .get_delta_mut(self)
    }

    fn token_maker_delta_mut(
        self,
        maker: Address,
        token_maker_deltas: &mut TokenMakerDeltas,
    ) -> Option<&mut UnsidedMakerDelta> {
        let key = TokenMakerDeltaKey {
            maker,
            token_index: self,
        };

        token_maker_deltas
            .hardcoded_token_deltas
            .get_or_insert_mut(key)
    }
}

impl MarketVariant for DynamicIndex {
    const DISCRIMINATOR: u8 = 1;

    type MarketKey<P: PairShape> = DynamicMarketKey<P>;

    fn token_sender_delta_mut(
        self,
        token_sender_deltas: &mut TokenSenderDeltas,
    ) -> &mut ERC20Delta {
        match self {
            DynamicIndex::Hardcoded(hardcoded_token_index) => token_sender_deltas
                .hardcoded_token_deltas
                .get_delta_mut(hardcoded_token_index),

            DynamicIndex::Custom(custom_token_index) => token_sender_deltas
                .custom_token_deltas
                .get_delta_mut(custom_token_index),
        }
    }

    fn token_maker_delta_mut(
        self,
        maker: Address,
        token_maker_deltas: &mut TokenMakerDeltas,
    ) -> Option<&mut UnsidedMakerDelta> {
        match self {
            DynamicIndex::Hardcoded(hardcoded_token_index) => {
                let key = TokenMakerDeltaKey {
                    maker,
                    token_index: hardcoded_token_index,
                };

                token_maker_deltas
                    .hardcoded_token_deltas
                    .get_or_insert_mut(key)
            }

            DynamicIndex::Custom(custom_token_index) => {
                let key = TokenMakerDeltaKey {
                    maker,
                    token_index: custom_token_index,
                };

                token_maker_deltas
                    .custom_token_deltas
                    .get_or_insert_mut(key)
            }
        }
    }
}
