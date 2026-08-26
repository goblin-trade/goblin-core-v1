use crate::{
    axis::{leg::leg_matcher::LegMatcher, update::UpdateMarker},
    goblin_error::GoblinError,
    quantities::TryIntoUnsidedDelta,
    settlement::local_delta::{LocalDeltaStore, LocalTake},
};

impl LocalDeltaStore for LocalTake {
    fn add_leg<UM: UpdateMarker, In: LegMatcher>(
        &mut self,
        lots: In::Lots,
    ) -> Result<(), GoblinError> {
        let delta_lots = lots.try_into_unsided_delta()?;
        let total_delta = In::get_leg_mut(&mut self.inner);

        *total_delta =
            UM::checked_update(*total_delta, delta_lots).ok_or(GoblinError::DeltaOverflow)?;

        Ok(())
    }
}
