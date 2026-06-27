use crate::{
    axis::{
        leg::{leg_matcher::LegMatcher, leg_reader::LegReader, SamePair},
        token::token_marker::TokenMarker,
    },
    quantities::{DeltaAtoms, UnsidedDeltaAtomsPerLot},
    settlement::local_delta_v3::LocalDeltaV3,
};

#[derive(Clone, Copy)]
pub struct TokenDeltaV3<T: TokenMarker> {
    pub deposit: T::GlobalDeposit,
    pub take: DeltaAtoms,
    pub make: DeltaAtoms,
}

impl<T: TokenMarker> TokenDeltaV3<T> {
    pub fn from_local_delta<In: LegReader>(
        atoms_per_lot_pair: &SamePair<UnsidedDeltaAtomsPerLot>,
        local_delta: &LocalDeltaV3,
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
