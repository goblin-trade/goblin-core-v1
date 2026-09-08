mod decrease;
mod increase;

use crate::{goblin_error::GoblinError, quantities::BaseLots, state::resting_order::RestingOrder};

pub trait UpdateMake {
    /// Update base lots in the resting order
    ///
    /// # Convention
    ///
    /// `UM` represents increase or decrease in trader balance.
    /// Therefore UM = Increase subtracts from resting order while UM = Decrease adds.
    fn update_resting_order(
        base_lots: BaseLots,
        resting_order: &mut RestingOrder,
    ) -> Result<BaseLots, GoblinError>;
}
