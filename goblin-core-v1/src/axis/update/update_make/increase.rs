use crate::{
    axis::update::{update_make::UpdateMake, Increase},
    goblin_error::GoblinError,
    quantities::BaseLots,
    state::resting_order::RestingOrder,
};

impl UpdateMake for Increase {
    fn update_resting_order(
        base_lots: BaseLots,
        resting_order: &mut RestingOrder,
    ) -> Result<BaseLots, GoblinError> {
        let delta_base_lots = resting_order.base_lots.min(base_lots);
        resting_order.base_lots -= delta_base_lots;
        Ok(delta_base_lots)
    }
}
