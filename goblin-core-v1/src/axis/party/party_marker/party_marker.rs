use crate::{
    axis::{
        leg::{leg_matcher::LegMatcher, SamePair},
        party::PartyEnum,
        token::token_quantity::TokenQuantity,
    },
    axis_helpers::{AxisMarker, LegToToken},
    goblin_error::GoblinError,
    market::{TokenIndexPair, TokenPair},
    quantities::UnsidedAtomsPerLot,
    settlement::{
        global_delta::GlobalDeltaStore,
        local_delta::{LocalDeltaStore, LocalDeposits},
    },
    types::StoreReader,
};

pub trait PartyMarker: AxisMarker<Enum = PartyEnum> {
    type LocalDeltaStore;

    type GlobalDeltaStore<TP, In>: GlobalDeltaStore<TP, In>
    where
        TP: TokenPair,
        In: LegMatcher
            + LegToToken<TP>
            + StoreReader<TokenIndexPair<TP>, Result = <In::Selected as TokenQuantity>::TokenIndex>
            + StoreReader<LocalDeposits<TP>, Result = <In::Selected as TokenQuantity>::LocalDeposit>;

    fn try_new<TP, In>(
        local_delta: &Self::LocalDeltaStore,
        local_deposits: &LocalDeposits<TP>,
        atoms_per_lot_pair: &SamePair<UnsidedAtomsPerLot>,
    ) -> Result<Self::GlobalDeltaStore<TP, In>, GoblinError>
    where
        TP: TokenPair,
        In: LegMatcher
            + LegToToken<TP>
            + StoreReader<TokenIndexPair<TP>, Result = <In::Selected as TokenQuantity>::TokenIndex>
            + StoreReader<LocalDeposits<TP>, Result = <In::Selected as TokenQuantity>::LocalDeposit>;
}
