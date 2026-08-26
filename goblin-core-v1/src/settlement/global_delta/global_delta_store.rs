use crate::{
    axis::{
        leg::{leg_matcher::LegMatcher, SamePair},
        token::token_quantity::TokenQuantity,
    },
    axis_helpers::LegToToken,
    goblin_error::GoblinError,
    market::{TokenIndexPair, TokenPair},
    quantities::UnsidedAtomsPerLot,
    settlement::local_delta::LocalDeposits,
    types::StoreReader,
};

pub trait GlobalDeltaStore: Sized {
    type LocalDeltaStore;

    fn try_new<TP, In>(
        local_delta: &Self::LocalDeltaStore,
        local_deposits: &LocalDeposits<TP>,
        atoms_per_lot_pair: &SamePair<UnsidedAtomsPerLot>,
    ) -> Result<Self, GoblinError>
    where
        TP: TokenPair,
        In: LegMatcher
            + LegToToken<TP>
            + StoreReader<TokenIndexPair<TP>, Result = <In::Selected as TokenQuantity>::TokenIndex>
            + StoreReader<LocalDeposits<TP>, Result = <In::Selected as TokenQuantity>::LocalDeposit>;
}
