use crate::{
    markets::PairShape,
    settlement::global_delta::{ERC20DeltaList, LazyERC20Delta, TokenDeltas},
    state::{DynamicMarketKey, HardcodedMarketKey, SlotKey},
    tokens::{DynamicIndex, HardcodedIndex},
};

/// We have 2 market variants
///
/// * Hardcoded market- has hardcoded tokens
/// * Dynamic market- has dynamic tokens that can be either dynamic or custom
pub trait MarketVariant {
    const DISCRIMINATOR: u8;

    /// Key to read market state slot
    type MarketKey<P: PairShape>: SlotKey;

    fn token_delta_mut(self, token_deltas: &mut TokenDeltas) -> &mut LazyERC20Delta;
}

impl MarketVariant for HardcodedIndex {
    const DISCRIMINATOR: u8 = 0;

    type MarketKey<P: PairShape> = HardcodedMarketKey<P>;

    fn token_delta_mut(self, token_deltas: &mut TokenDeltas) -> &mut LazyERC20Delta {
        token_deltas.hardcoded_token_deltas.get_delta_mut(self)
    }
}

impl MarketVariant for DynamicIndex {
    const DISCRIMINATOR: u8 = 1;

    type MarketKey<P: PairShape> = DynamicMarketKey<P>;

    fn token_delta_mut(self, token_deltas: &mut TokenDeltas) -> &mut LazyERC20Delta {
        match self {
            DynamicIndex::Hardcoded(hardcoded_token_index) => token_deltas
                .hardcoded_token_deltas
                .get_delta_mut(hardcoded_token_index),

            DynamicIndex::Custom(custom_token_index) => token_deltas
                .custom_token_deltas
                .get_delta_mut(custom_token_index),
        }
    }
}
