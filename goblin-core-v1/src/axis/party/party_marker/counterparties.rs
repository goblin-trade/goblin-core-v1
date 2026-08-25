use crate::{
    axis::{
        leg::leg_matcher::LegMatcher,
        party::{Counterparties, PartyMarker, Sender},
        update::UpdateMarker,
    },
    goblin_error::GoblinError,
    quantities::TryIntoUnsidedDelta,
    settlement::{local_delta::LocalDelta, CheckedOps},
    types::{Address, StoreReader},
};

impl PartyMarker for Counterparties {
    fn add_local_delta<UM: UpdateMarker, In: LegMatcher>(
        lots: In::Lots,
        counterparty: &Address,
        local_delta: &mut LocalDelta,
    ) -> Result<(), GoblinError> {
        let sender_take_for_leg = In::get_leg_mut(&mut Sender::get_leg_mut(local_delta).take);

        *sender_take_for_leg = sender_take_for_leg
            .checked_add(in_delta_lots)
            .ok_or(GoblinError::DeltaOverflow)?;

        Ok(())
    }
}
