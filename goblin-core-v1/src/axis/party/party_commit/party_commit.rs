use crate::{
    axis::{leg::SamePair, party::PartyDelta},
    axis_helpers::{PairLeg, TokenPair},
    goblin_error::GoblinError,
    market::TokenIndexPair,
    quantities::UnsidedAtomsPerLot,
    settlement::{global_delta::GlobalDelta, local_delta::LocalUpdate, CheckedOps},
};

pub trait PartyCommit: PartyDelta {
    fn commit_local_delta<'a, TP: TokenPair>(
        params: (&TokenIndexPair<TP>, &SamePair<UnsidedAtomsPerLot>),
        local_update: &LocalUpdate<TP>,
        global_delta: &mut GlobalDelta,
    ) -> Result<(), GoblinError>;

    fn commit_leg<'a, PL: PairLeg>(
        address: &Self::Address,
        params: (&TokenIndexPair<PL::Pair>, &SamePair<UnsidedAtomsPerLot>),
        local: Self::LocalUpdate<'a, PL::Pair>,
        global_delta: &mut GlobalDelta,
    ) -> Result<(), GoblinError> {
        let (token_index_pair, atoms_per_lot_pair) = params;
        let new_delta = Self::try_new::<PL>(local, atoms_per_lot_pair)?;
        let delta_store = Self::get_store::<PL>(address, token_index_pair, global_delta)?;

        *delta_store = delta_store
            .checked_add(new_delta)
            .ok_or(GoblinError::DeltaOverflow)?;

        Ok(())
    }
}
