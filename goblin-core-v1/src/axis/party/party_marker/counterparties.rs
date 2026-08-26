use crate::{
    axis::{
        leg::{leg_matcher::LegMatcher, SamePair},
        party::{party_marker::party_marker::PartyMarker, Counterparties},
        token::token_quantity::TokenQuantity,
    },
    axis_helpers::LegToToken,
    goblin_error::GoblinError,
    market::{TokenIndexPair, TokenPair},
    quantities::UnsidedAtomsPerLot,
    settlement::{
        global_delta::GlobalCounterparty,
        local_delta::{LocalCounterparty, LocalDeposits},
    },
    types::StoreReader,
};

impl PartyMarker for Counterparties {
    type LocalDeltaStore = LocalCounterparty;

    type GlobalDeltaStore<TP, In>
        = GlobalCounterparty
    where
        TP: TokenPair,
        In: LegMatcher
            + LegToToken<TP>
            + StoreReader<TokenIndexPair<TP>, Result = <In::Selected as TokenQuantity>::TokenIndex>
            + StoreReader<LocalDeposits<TP>, Result = <In::Selected as TokenQuantity>::LocalDeposit>;

    fn try_new<TP, In>(
        local_delta: &Self::LocalDeltaStore,
        _local_deposits: &LocalDeposits<TP>,
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
}
