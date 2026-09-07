use crate::{
    axis::update::{update_make::UpdateMake, Decrease},
    goblin_error::GoblinError,
    quantities::BaseLots,
    settlement::CheckedOps,
    state::resting_order::RestingOrder,
};

impl UpdateMake for Decrease {
    fn update_resting_order(
        base_lots: BaseLots,
        resting_order: &mut RestingOrder,
    ) -> Result<BaseLots, GoblinError> {
        let stored_base_lots = &mut resting_order.base_lots;
        *stored_base_lots = stored_base_lots
            .checked_add(base_lots)
            .ok_or(GoblinError::Overflow)?;

        Ok(base_lots)
    }
}
