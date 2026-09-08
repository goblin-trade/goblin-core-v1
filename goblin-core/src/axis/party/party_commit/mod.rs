mod counterparties;
mod sender;

use super::PartyDelta;

use crate::{
    axis_helpers::{PairLeg, TokenPair},
    goblin_error::GoblinError,
    market::CommonMarket,
    settlement::{CheckedOps, global_delta::GlobalDelta, local_delta::LocalUpdate},
};

pub trait PartyCommit: PartyDelta {
    fn commit<TP: TokenPair>(
        market: &CommonMarket<TP>,
        local_update: &LocalUpdate<TP>,
        global_delta: &mut GlobalDelta,
    ) -> Result<(), GoblinError>;

    fn commit_leg<'a, PL: PairLeg>(
        address: &Self::Address,
        market: &CommonMarket<PL::Pair>,
        local: Self::LocalUpdate<'a, PL::Pair>,
        global_delta: &mut GlobalDelta,
    ) -> Result<(), GoblinError> {
        let new_delta = Self::try_new::<PL>(local, &market.atoms_per_lot_pair())?;
        let delta_store = Self::get_store::<PL>(address, &market.token_index_pair, global_delta)?;

        *delta_store = delta_store
            .checked_add(new_delta)
            .ok_or(GoblinError::DeltaOverflow)?;

        Ok(())
    }
}
