use crate::{
    markets::{DynamicMarket, HardcodedMarket, PairShape},
    settlement::global::{DeltaListTrait, ERC20DeltaMaybe, GlobalDelta},
    state::{DynamicMarketKey, HardcodedMarketKey, SlotKey},
    tokens::{DynamicIndex, HardcodedIndex},
};

/// We have 2 market variants
///
/// * Hardcoded market- has hardcoded tokens
/// * Dynamic market- has dynamic tokens that can be either dynamic or custom
pub trait MarketVariant {
    const DISCRIMINATOR: u8;

    /// The underlying market type
    type Market<P: PairShape>;

    /// Key to read market state slot
    type MarketKey<P: PairShape>: SlotKey;

    fn token_delta_mut(self, global_delta: &mut GlobalDelta) -> &mut ERC20DeltaMaybe;
}

impl MarketVariant for HardcodedIndex {
    const DISCRIMINATOR: u8 = 0;

    type Market<P: PairShape> = HardcodedMarket<P>;

    type MarketKey<P: PairShape> = HardcodedMarketKey<P>;

    fn token_delta_mut(self, global_delta: &mut GlobalDelta) -> &mut ERC20DeltaMaybe {
        global_delta.hardcoded_token_deltas.get_delta_mut(self)
    }
}

impl MarketVariant for DynamicIndex {
    const DISCRIMINATOR: u8 = 1;

    type Market<P: PairShape> = DynamicMarket<P>;

    type MarketKey<P: PairShape> = DynamicMarketKey<P>;

    fn token_delta_mut(self, global_delta: &mut GlobalDelta) -> &mut ERC20DeltaMaybe {
        match self {
            DynamicIndex::Hardcoded(hardcoded_token_index) => global_delta
                .hardcoded_token_deltas
                .get_delta_mut(hardcoded_token_index),

            DynamicIndex::Custom(custom_token_index) => global_delta
                .custom_token_deltas
                .get_delta_mut(custom_token_index),
        }
    }
}
