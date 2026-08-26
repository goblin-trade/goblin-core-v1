use crate::{
    axis::{
        leg::{leg_matcher::LegMatcher, SamePair},
        token::{token_marker::TokenMarker, token_quantity::TokenQuantity},
    },
    axis_helpers::LegToToken,
    goblin_error::GoblinError,
    market::{TokenIndexPair, TokenPair},
    quantities::{UnsidedAtomsPerLot, UnsidedDeltaAtomsPerLot},
    settlement::{
        global_delta::{GlobalDeltaStore, TokenDelta},
        local_delta::{LocalDeposits, LocalSender},
    },
    types::StoreReader,
};

impl<T, TP, In> GlobalDeltaStore<TP, In> for TokenDelta<T>
where
    T: TokenMarker,
    TP: TokenPair,
    In: LegMatcher
        + LegToToken<TP, Selected = T>
        + StoreReader<TokenIndexPair<TP>, Result = <In::Selected as TokenQuantity>::TokenIndex>
        + StoreReader<LocalDeposits<TP>, Result = <In::Selected as TokenQuantity>::LocalDeposit>,
{
    type LocalDeltaStore = LocalSender;

    fn try_new(
        local_delta: &Self::LocalDeltaStore,
        local_deposits: &LocalDeposits<TP>,
        atoms_per_lot_pair: &SamePair<UnsidedAtomsPerLot>,
    ) -> Result<Self, GoblinError> {
        let local_deposit = In::get(local_deposits);
        let delta_atoms_per_lot_pair =
            SamePair::<UnsidedDeltaAtomsPerLot>::try_from(atoms_per_lot_pair)?;

        let atoms_per_lot = In::get(&delta_atoms_per_lot_pair);

        let deposit = T::get_global_deposit(local_deposit, atoms_per_lot);
        let local_take = In::get(&local_delta.take.inner);

        // TODO checked mul?
        let take = local_take * atoms_per_lot;

        let local_make = In::get(&local_delta.make.inner);
        let make = local_make * atoms_per_lot;

        Ok(Self {
            deposit,
            take,
            make,
        })
    }
}
