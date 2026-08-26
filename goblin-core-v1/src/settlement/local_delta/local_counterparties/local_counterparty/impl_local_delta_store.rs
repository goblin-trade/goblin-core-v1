use crate::{
    axis::{leg::leg_matcher::LegMatcher, update::UpdateMarker},
    goblin_error::GoblinError,
    quantities::UnsideQuantity,
    settlement::{
        local_delta::{LocalCounterparty, LocalDeltaStore},
        CheckedOps,
    },
};

impl LocalDeltaStore for LocalCounterparty {
    fn add_leg<UM: UpdateMarker, In: LegMatcher>(
        &mut self,
        lots: In::Lots,
    ) -> Result<(), GoblinError> {
        let total_delta = UM::get_leg_mut(In::get_leg_mut(self));

        *total_delta = total_delta
            .checked_add(lots.unsided())
            .ok_or(GoblinError::DeltaOverflow)?;

        Ok(())
    }
}
