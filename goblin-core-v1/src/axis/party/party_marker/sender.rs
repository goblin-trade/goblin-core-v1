use crate::{
    axis::{
        leg::{leg_matcher::LegMatcher, leg_quantities::LegQuantities},
        party::{PartyMarker, Sender},
        update::{Decrease, Increase},
    },
    goblin_error::GoblinError,
    quantities::TryIntoUnsidedDelta,
    settlement::{local_delta::LocalDelta, CheckedOps},
    types::{Address, StoreReader},
};

impl PartyMarker for Sender {
    fn add_local_delta<In: LegMatcher>(
        lots: In::Lots,
        lots_opposite: <In::Opposite as LegQuantities>::Lots,
        _counterparty: &Address,
        local_delta: &mut LocalDelta,
    ) -> Result<(), GoblinError> {
        let in_delta_lots = lots.try_into_unsided_delta::<Decrease>()?;
        let out_delta_lots = lots_opposite.try_into_unsided_delta::<Increase>()?;

        let local_take = &mut Sender::get_leg_mut(local_delta).take;

        // TODO build an inner function to reduce duplication
        let sender_in = In::get_leg_mut(local_take);
        *sender_in = sender_in
            .checked_add(in_delta_lots)
            .ok_or(GoblinError::DeltaOverflow)?;

        let sender_out = In::Opposite::get_leg_mut(local_take);
        *sender_out = sender_out
            .checked_add(out_delta_lots)
            .ok_or(GoblinError::DeltaOverflow)?;

        Ok(())
    }
}
