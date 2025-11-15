use crate::{
    markets::PairShape,
    settlement::{DeltaListTrait, ERC20DeltaMaybe, SenderBalanceUpdates},
    state::{DynamicMarketKey, HardcodedMarketKey, SlotKey},
    tokens::{DynamicIndex, HardcodedIndex},
};

/// We have 2 market variants
///
/// * Hardcoded market- has hardcoded tokens
/// * Dynamic market- has dynamic tokens that can be either dynamic or custom
pub trait MarketVariant {
    const DISCRIMINATOR: u8;

    /// Key to read market slot
    type MarketKey<P: PairShape>: SlotKey;

    fn token_delta_mut(
        self,
        sender_balance_updates: &mut SenderBalanceUpdates,
    ) -> &mut ERC20DeltaMaybe;
}

impl MarketVariant for HardcodedIndex {
    const DISCRIMINATOR: u8 = 0;

    type MarketKey<P: PairShape> = HardcodedMarketKey<P>;

    fn token_delta_mut(
        self,
        sender_balance_updates: &mut SenderBalanceUpdates,
    ) -> &mut ERC20DeltaMaybe {
        sender_balance_updates
            .hardcoded_token_deltas
            .get_delta_mut(self)
    }
}

impl MarketVariant for DynamicIndex {
    const DISCRIMINATOR: u8 = 1;

    type MarketKey<P: PairShape> = DynamicMarketKey<P>;

    fn token_delta_mut(
        self,
        sender_balance_updates: &mut SenderBalanceUpdates,
    ) -> &mut ERC20DeltaMaybe {
        match self {
            DynamicIndex::Hardcoded(index) => sender_balance_updates
                .hardcoded_token_deltas
                .get_delta_mut(index),

            DynamicIndex::Custom(index) => sender_balance_updates
                .custom_token_deltas
                .get_delta_mut(index),
        }
    }
}
