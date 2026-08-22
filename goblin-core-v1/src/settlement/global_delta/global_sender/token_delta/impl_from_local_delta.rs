use crate::{
    axis::{
        leg::{leg_matcher::LegMatcher, SamePair},
        token::token_marker::TokenMarker,
    },
    quantities::UnsidedDeltaAtomsPerLot,
    settlement::{global_delta::TokenDelta, local_delta::LocalSender},
};

pub trait FromLocalDelta<T> {
    fn from_local<In>(
        local_deposit: T::LocalDeposit,
        local_sender: &LocalSender,
        atoms_per_lot_pair: &SamePair<UnsidedDeltaAtomsPerLot>,
    ) -> Self
    where
        T: TokenMarker,
        In: LegMatcher;
}

impl<T> FromLocalDelta<T> for TokenDelta<T>
where
    T: TokenMarker,
{
    fn from_local<In>(
        local_deposit: T::LocalDeposit,
        local_delta: &LocalSender,
        atoms_per_lot_pair: &SamePair<UnsidedDeltaAtomsPerLot>,
    ) -> Self
    where
        T: TokenMarker,
        In: LegMatcher,
    {
        let atoms_per_lot = In::get(atoms_per_lot_pair);
        let deposit = T::get_global_deposit(local_deposit, atoms_per_lot);

        // TODO checked mul?
        let local_take = In::get(&local_delta.take);
        let take = local_take * atoms_per_lot;

        let local_make = In::get(&local_delta.make.inner);
        let make = local_make * atoms_per_lot;

        Self {
            deposit,
            take,
            make,
        }
    }
}
