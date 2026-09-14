use crate::{
    axis::update::{Increase, update_make::UpdateMake},
    goblin_error::GoblinError,
    quantities::BaseLots,
    state::resting_order::RestingOrder,
};

impl UpdateMake for Increase {
    fn update_resting_order(
        base_lots: BaseLots<u64>,
        resting_order: &mut RestingOrder,
    ) -> Result<BaseLots<u64>, GoblinError> {
        let delta_base_lots = resting_order.base_lots.min(base_lots);
        resting_order.base_lots -= delta_base_lots;
        Ok(delta_base_lots)
    }
}
