use crate::{
    axis::{
        leg::SamePair,
        party::{party_marker::party_marker::PartyMarker, Counterparties},
    },
    axis_helpers::{PairLeg, TokenPair},
    for_axes,
    goblin_error::GoblinError,
    market::TokenIndexPair,
    quantities::UnsidedAtomsPerLot,
    settlement::{
        global_delta::{CounterpartyTokenKey, GlobalCounterparty, GlobalDelta},
        local_delta::{LocalCounterparty, LocalDelta, LocalDeposits},
    },
    types::{Address, StoreReader},
};

impl PartyMarker for Counterparties {
    type Address = Address;

    type Local<'a, TP: TokenPair> = &'a LocalCounterparty;
    type GlobalDeltaStore<PL: PairLeg> = GlobalCounterparty;

    fn try_new<'a, PL: PairLeg>(
        local_delta: Self::Local<'a, PL::Pair>,
        atoms_per_lot_pair: &SamePair<UnsidedAtomsPerLot>,
    ) -> Result<Self::GlobalDeltaStore<PL>, GoblinError> {
        let local_counterparty = PL::Leg::get(local_delta);
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

    fn commit_local_delta<'a, TP: TokenPair>(
        atoms_per_lot_pair: &SamePair<UnsidedAtomsPerLot>,
        token_index_pair: &TokenIndexPair<TP>,
        local_delta: &LocalDelta<'a>,
        _local_deposits: &LocalDeposits<TP>,
        global_delta: &mut GlobalDelta,
    ) -> Result<(), GoblinError> {
        let local_counterparties = &**Counterparties::get_leg(local_delta);
        for (address, local_counterparty) in local_counterparties.into_iter() {
            for_axes!(In => Counterparties::commit_leg::<(TP, In)>(
                address,
                &local_counterparty,
                atoms_per_lot_pair,
                token_index_pair,
                global_delta
            )?);
        }

        Ok(())
    }
}
