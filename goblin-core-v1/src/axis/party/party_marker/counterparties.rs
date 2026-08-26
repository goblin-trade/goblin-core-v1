use crate::{
    axis::{
        leg::{leg_matcher::LegMatcher, SamePair},
        party::{party_marker::party_marker::PartyMarker, Counterparties},
        token::token_quantity::TokenQuantity,
    },
    axis_helpers::LegToToken,
    for_axes,
    goblin_error::GoblinError,
    market::{TokenIndexPair, TokenPair},
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

    type GlobalDeltaStore<TP, In>
        = GlobalCounterparty
    where
        TP: TokenPair,
        In: LegMatcher
            + LegToToken<TP>
            + StoreReader<TokenIndexPair<TP>, Result = <In::Selected as TokenQuantity>::TokenIndex>
            + StoreReader<LocalDeposits<TP>, Result = <In::Selected as TokenQuantity>::LocalDeposit>;

    fn try_new<'a, TP, In>(
        local_delta: Self::Local<'a, TP>,
        atoms_per_lot_pair: &SamePair<UnsidedAtomsPerLot>,
    ) -> Result<Self::GlobalDeltaStore<TP, In>, GoblinError>
    where
        TP: TokenPair,
        In: LegMatcher
            + LegToToken<TP>
            + StoreReader<TokenIndexPair<TP>, Result = <In::Selected as TokenQuantity>::TokenIndex>
            + StoreReader<LocalDeposits<TP>, Result = <In::Selected as TokenQuantity>::LocalDeposit>,
    {
        let local_counterparty = In::get(local_delta);
        let atoms_per_lot = In::get(atoms_per_lot_pair);
        let atoms_pair = atoms_per_lot * local_counterparty;

        Ok(GlobalCounterparty { inner: atoms_pair })
    }

    fn get_store<'a, TP, In>(
        address: &Self::Address,
        token_index_pair: &TokenIndexPair<TP>,
        global_delta: &'a mut GlobalDelta,
    ) -> Result<&'a mut Self::GlobalDeltaStore<TP, In>, GoblinError>
    where
        TP: TokenPair,
        In: LegMatcher
            + LegToToken<TP>
            + StoreReader<TokenIndexPair<TP>, Result = <In::Selected as TokenQuantity>::TokenIndex>
            + StoreReader<LocalDeposits<TP>, Result = <In::Selected as TokenQuantity>::LocalDeposit>,
    {
        let counterparty_delta = Counterparties::get_leg_mut(global_delta);

        let key = CounterpartyTokenKey {
            counterparty: *address,
            token_index: In::get(token_index_pair),
        };

        In::Selected::get_leg_mut(counterparty_delta)
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
            for_axes!(In => Counterparties::commit_leg::<TP, In>(
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
