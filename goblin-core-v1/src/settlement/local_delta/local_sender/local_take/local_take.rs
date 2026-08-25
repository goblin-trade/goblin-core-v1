use goblin_macros::ConstDefault;

use crate::{
    axis::{
        leg::{leg_matcher::LegMatcher, SamePair},
        update::UpdateMarker,
    },
    goblin_error::GoblinError,
    quantities::{TryIntoUnsidedDelta, UnsidedDeltaLots},
    settlement::{local_delta::LocalDeltaStore, CheckedOps},
};

#[derive(ConstDefault, Clone, Copy)]
pub struct LocalTake {
    pub inner: SamePair<UnsidedDeltaLots>,
}

impl LocalDeltaStore for LocalTake {
    fn add_leg<UM: UpdateMarker, In: LegMatcher>(
        &mut self,
        lots: In::Lots,
    ) -> Result<(), GoblinError> {
        let delta_lots = lots.try_into_unsided_delta::<UM>()?;

        let total_delta = In::get_leg_mut(&mut self.inner);
        *total_delta = total_delta
            .checked_add(delta_lots)
            .ok_or(GoblinError::DeltaOverflow)?;

        Ok(())
    }
}
