use crate::{
    axis::{leg::SamePair, party::PartyEnum},
    axis_helpers::{AxisMarker, PairLeg, TokenPair},
    goblin_error::GoblinError,
    market::TokenIndexPair,
    quantities::UnsidedAtomsPerLot,
    settlement::{
        global_delta::GlobalDelta,
        local_delta::{LocalDelta, LocalDeposits},
        CheckedOps,
    },
};

/// 3 sub traits
///
/// - PartyDelta: local, global, try_new, get_store
/// - PartyCommit
/// - PartySettle
pub trait PartyMarker: AxisMarker<Enum = PartyEnum> {
    type Address;

    /// The local type that is converted into GlobalDeltaStore
    type Local<'a, TP: TokenPair>;

    type GlobalDeltaStore<PL: PairLeg>: CheckedOps + Clone + Copy;

    fn try_new<'a, PL: PairLeg>(
        local: Self::Local<'a, PL::Pair>,
        atoms_per_lot_pair: &SamePair<UnsidedAtomsPerLot>,
    ) -> Result<Self::GlobalDeltaStore<PL>, GoblinError>;

    fn get_store<'a, PL: PairLeg>(
        address: &Self::Address,
        token_index_pair: &TokenIndexPair<PL::Pair>,
        global_delta: &'a mut GlobalDelta,
    ) -> Result<&'a mut Self::GlobalDeltaStore<PL>, GoblinError>;

    fn commit_local_delta<'a, TP: TokenPair>(
        params: (&TokenIndexPair<TP>, &SamePair<UnsidedAtomsPerLot>),
        local: (&LocalDelta<'a>, &LocalDeposits<TP>),
        global_delta: &mut GlobalDelta,
    ) -> Result<(), GoblinError>;

    fn commit_leg<'a, PL: PairLeg>(
        address: &Self::Address,
        params: (&TokenIndexPair<PL::Pair>, &SamePair<UnsidedAtomsPerLot>),
        local: Self::Local<'a, PL::Pair>,
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
