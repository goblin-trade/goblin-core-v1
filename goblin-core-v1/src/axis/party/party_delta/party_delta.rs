use crate::{
    axis::leg::SamePair,
    axis_helpers::{PairLeg, TokenPair},
    goblin_error::GoblinError,
    market::TokenIndexPair,
    quantities::UnsidedAtomsPerLot,
    settlement::{global_delta::GlobalDelta, CheckedOps},
};

pub trait PartyDelta {
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
}
