use crate::{
    axis::{
        leg::leg_matcher::LegMatcher,
        party::{PartyMarker, Sender},
        update::UpdateMarker,
    },
    goblin_error::GoblinError,
    quantities::TryIntoUnsidedDelta,
    settlement::{local_delta::LocalDelta, CheckedOps},
    types::{Address, StoreReader},
};

impl PartyMarker for Sender {
    fn add_local_delta<UM: UpdateMarker, In: LegMatcher>(
        lots: In::Lots,
        _counterparty: &Address,
        local_delta: &mut LocalDelta,
    ) -> Result<(), GoblinError> {
        let in_delta_lots = lots.try_into_unsided_delta::<UM>()?;

        let sender_take_for_leg = In::get_leg_mut(&mut Sender::get_leg_mut(local_delta).take);

        *sender_take_for_leg = sender_take_for_leg
            .checked_add(in_delta_lots)
            .ok_or(GoblinError::DeltaOverflow)?;

        Ok(())
    }
}
