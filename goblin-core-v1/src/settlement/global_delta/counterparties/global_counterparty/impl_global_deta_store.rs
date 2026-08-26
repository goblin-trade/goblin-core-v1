use crate::{
    axis::leg::{leg_matcher::LegMatcher, SamePair},
    goblin_error::GoblinError,
    market::TokenPair,
    quantities::UnsidedAtomsPerLot,
    settlement::{
        global_delta::{GlobalCounterparty, GlobalDeltaStore},
        local_delta::{LocalCounterparty, LocalDeposits},
    },
};

impl GlobalDeltaStore for GlobalCounterparty {
    type LocalDeltaStore = LocalCounterparty;

    fn try_new<TP, In>(
        local_delta: &Self::LocalDeltaStore,
        _local_deposits: &LocalDeposits<TP>,
        atoms_per_lot_pair: &SamePair<UnsidedAtomsPerLot>,
    ) -> Result<Self, GoblinError>
    where
        TP: TokenPair,
        In: LegMatcher,
    {
        let local_counterparty = In::get(local_delta);
        let atoms_per_lot = In::get(atoms_per_lot_pair);
        let atoms_pair = atoms_per_lot * local_counterparty;

        Ok(Self { inner: atoms_pair })
    }
}
