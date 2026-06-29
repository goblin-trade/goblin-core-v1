use crate::{
    axis::{
        leg::{leg_reader::LegReader, SamePair},
        token::token_marker::TokenMarker,
    },
    goblin_error::GoblinError,
    quantities::{UnsidedDeltaAtoms, UnsidedDeltaAtomsPerLot},
    settlement::local_delta::LocalDelta,
    state::Store,
};

#[derive(Clone, Copy)]
pub struct TokenDelta<T: TokenMarker> {
    pub deposit: T::GlobalDeposit,
    pub take: UnsidedDeltaAtoms,
    pub make: UnsidedDeltaAtoms,
}

impl<T: TokenMarker> TokenDelta<T> {
    pub fn from_local_delta<In: LegReader>(
        atoms_per_lot_pair: &SamePair<UnsidedDeltaAtomsPerLot>,
        local_delta: &LocalDelta,
    ) -> Self {
        let atoms_per_lot = In::get(atoms_per_lot_pair);

        let local_deposit = T::get(In::get_leg(&local_delta.deposits));
        let deposit = T::get_global_deposit(local_deposit, atoms_per_lot);

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
