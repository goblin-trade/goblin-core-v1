use crate::{
    axis::{
        leg::SamePair,
        party::{Counterparties, PartyDelta},
    },
    axis_helpers::{PairLeg, TokenPair},
    goblin_error::GoblinError,
    market::TokenIndexPair,
    quantities::UnsidedAtomsPerLot,
    settlement::{
        global_delta::{CounterpartyTokenKey, GlobalCounterparty, GlobalDelta},
        local_delta::LocalCounterparty,
    },
    types::{Address, StoreReader},
};

impl PartyDelta for Counterparties {
    type Address = Address;

    type Local<'a, TP: TokenPair> = &'a LocalCounterparty;
    type GlobalDeltaStore<PL: PairLeg> = GlobalCounterparty;

    fn try_new<'a, PL: PairLeg>(
        local: Self::Local<'a, PL::Pair>,
        atoms_per_lot_pair: &SamePair<UnsidedAtomsPerLot>,
    ) -> Result<Self::GlobalDeltaStore<PL>, GoblinError> {
        let local_counterparty = PL::Leg::get(local);
        let atoms_per_lot = PL::Leg::get(atoms_per_lot_pair);
        let atoms_pair = atoms_per_lot * local_counterparty;

        Ok(GlobalCounterparty { inner: atoms_pair })
    }

    fn get_store<'a, PL: PairLeg>(
        address: &Self::Address,
        token_index_pair: &TokenIndexPair<PL::Pair>,
        global_delta: &'a mut GlobalDelta,
    ) -> Result<&'a mut Self::GlobalDeltaStore<PL>, GoblinError> {
        let counterparty_delta = Counterparties::get_leg_mut(global_delta);

        let key = CounterpartyTokenKey {
            counterparty: *address,
            token_index: PL::Leg::get(token_index_pair),
        };

        PL::Selected::get_leg_mut(counterparty_delta)
            .get_or_insert_mut(key)
            .ok_or(GoblinError::GlobalCounterpartyFull)
    }
}
