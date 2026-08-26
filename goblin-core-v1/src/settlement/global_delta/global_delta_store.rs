use crate::{
    axis::{
        leg::{leg_matcher::LegMatcher, SamePair},
        token::token_quantity::TokenQuantity,
    },
    axis_helpers::LegToToken,
    goblin_error::GoblinError,
    market::{TokenIndexPair, TokenPair},
    quantities::UnsidedAtomsPerLot,
    settlement::{global_delta::GlobalDelta, local_delta::LocalDeposits},
    types::{Address, StoreReader},
};

pub trait GlobalDeltaStore<TP, In>: Sized
where
    TP: TokenPair,
    In: LegMatcher
        + LegToToken<TP>
        + StoreReader<TokenIndexPair<TP>, Result = <In::Selected as TokenQuantity>::TokenIndex>
        + StoreReader<LocalDeposits<TP>, Result = <In::Selected as TokenQuantity>::LocalDeposit>,
{
    type LocalDeltaStore;

    fn try_new(
        local_delta: &Self::LocalDeltaStore,
        local_deposits: &LocalDeposits<TP>,
        atoms_per_lot_pair: &SamePair<UnsidedAtomsPerLot>,
    ) -> Result<Self, GoblinError>;

    fn get_store<'a>(
        address: &Address,
        token_index_pair: &TokenIndexPair<TP>,
        global_delta: &'a mut GlobalDelta,
    ) -> Result<&'a mut Self, GoblinError>;
}
