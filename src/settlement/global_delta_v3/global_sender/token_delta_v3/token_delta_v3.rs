use crate::{
    axis::{leg::leg_matcher::LegMatcher, token::token_marker::TokenMarker},
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
    pub fn from_local_delta<In: LegMatcher>(
        unsided_delta_atoms_per_lot: UnsidedDeltaAtomsPerLot,
        local_delta: &LocalDeltaV3,
    ) -> Self {
        let local_deposit = T::get(In::get_leg(&local_delta.deposits));
        let deposit = T::get_global_deposit(local_deposit, unsided_delta_atoms_per_lot);

        let local_take = In::get(&local_delta.take.sender);
        let take = local_take * unsided_delta_atoms_per_lot;

        let local_make = In::get(&local_delta.make.inner);
        let make = local_make * unsided_delta_atoms_per_lot;

        Self {
            deposit,
            take,
            make,
        }
    }
}
