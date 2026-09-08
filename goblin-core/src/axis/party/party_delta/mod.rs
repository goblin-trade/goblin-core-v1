mod counterparties;
mod sender;

use crate::{
    axis::leg::SamePair,
    axis_helpers::{PairLeg, TokenPair},
    goblin_error::GoblinError,
    market::TokenIndexPair,
    quantities::UnsidedAtomsPerLot,
    settlement::{CheckedOps, global_delta::GlobalDelta},
};

pub trait PartyDelta {
    type Address;

    type LocalUpdate<'a, TP: TokenPair>;

    type GlobalInner<PL: PairLeg>: CheckedOps + Clone + Copy;

    fn try_new<'a, PL: PairLeg>(
        local_update: Self::LocalUpdate<'a, PL::Pair>,
        atoms_per_lot_pair: &SamePair<UnsidedAtomsPerLot>,
    ) -> Result<Self::GlobalInner<PL>, GoblinError>;

    fn get_store<'a, PL: PairLeg>(
        address: &Self::Address,
        token_index_pair: &TokenIndexPair<PL::Pair>,
        global_delta: &'a mut GlobalDelta,
    ) -> Result<&'a mut Self::GlobalInner<PL>, GoblinError>;
}
