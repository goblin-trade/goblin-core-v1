use crate::{
    axis::{
        leg::{leg_reader::LegReader, SamePair},
        token::token_marker::TokenMarker,
    },
    quantities::UnsidedDeltaAtomsPerLot,
    settlement::{global_delta::TokenDelta, local_delta::LocalDelta},
};
pub trait FromLocalDelta {
    fn from_local_delta<In: LegReader>(
        local_delta: &LocalDelta,
        atoms_per_lot_pair: &SamePair<UnsidedDeltaAtomsPerLot>,
    ) -> Self;
}

impl<T: TokenMarker> FromLocalDelta for TokenDelta<T> {
    fn from_local_delta<In: LegReader>(
        local_delta: &LocalDelta,
        atoms_per_lot_pair: &SamePair<UnsidedDeltaAtomsPerLot>,
    ) -> Self {
        let atoms_per_lot = In::get(atoms_per_lot_pair);

        // TODO improved chaining syntax for nested tuples
        let local_deposit = T::get(In::get_leg(&local_delta.deposits));
        let deposit = T::get_global_deposit(local_deposit, atoms_per_lot);

        // TODO checked mul?
        let local_take = In::get(&local_delta.take.sender);
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
