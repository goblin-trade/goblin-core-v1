use goblin_macros::ConstDefault;

use crate::{
    axis::{
        leg::{leg_matcher::LegMatcher, SamePair},
        update::UpdateMarker,
    },
    goblin_error::GoblinError,
    quantities::{TryIntoUnsidedDelta, UnsidedDeltaLots},
    settlement::CheckedOps,
};

#[derive(ConstDefault, Clone, Copy)]
pub struct LocalTake {
    pub inner: SamePair<UnsidedDeltaLots>,
}

impl LocalTake {
    pub fn add_leg<UM, In>(&mut self, lots: In::Lots) -> Result<(), GoblinError>
    where
        UM: UpdateMarker,
        In: LegMatcher,
    {
        let delta_lots = lots.try_into_unsided_delta::<UM>()?;

        let total_delta = In::get_leg_mut(&mut self.inner);
        *total_delta = total_delta
            .checked_add(delta_lots)
            .ok_or(GoblinError::DeltaOverflow)?;

        Ok(())
    }
}
