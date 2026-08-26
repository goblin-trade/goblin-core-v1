use crate::{
    axis::{
        leg::{leg_matcher::LegMatcher, SamePair},
        token::token_quantity::TokenQuantity,
    },
    axis_helpers::LegToToken,
    goblin_error::GoblinError,
    market::{TokenIndexPair, TokenPair},
    quantities::UnsidedAtomsPerLot,
    settlement::{
        global_delta::{GlobalCounterparty, GlobalDeltaStore},
        local_delta::{LocalCounterparty, LocalDeposits},
    },
    types::StoreReader,
};

impl<TP, In> GlobalDeltaStore<TP, In> for GlobalCounterparty
where
    TP: TokenPair,
    In: LegMatcher
        + LegToToken<TP>
        + StoreReader<TokenIndexPair<TP>, Result = <In::Selected as TokenQuantity>::TokenIndex>
        + StoreReader<LocalDeposits<TP>, Result = <In::Selected as TokenQuantity>::LocalDeposit>,
{
    type LocalDeltaStore = LocalCounterparty;

    fn try_new(
        local_delta: &Self::LocalDeltaStore,
        _local_deposits: &LocalDeposits<TP>,
        atoms_per_lot_pair: &SamePair<UnsidedAtomsPerLot>,
    ) -> Result<Self, GoblinError> {
        let local_counterparty = In::get(local_delta);
        let atoms_per_lot = In::get(atoms_per_lot_pair);
        let atoms_pair = atoms_per_lot * local_counterparty;

        Ok(Self { inner: atoms_pair })
    }
}
