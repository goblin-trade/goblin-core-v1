use crate::state::{InnerIndex, RestingOrderIndex, Side};

use super::order_id::OrderId;

// TODO move to same file having OrderId
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GroupPosition {
    pub inner_index: InnerIndex,
    pub resting_order_index: RestingOrderIndex,
}

impl GroupPosition {
    /// Calculate the starting position for GroupPositionIterator
    pub fn count(&self, side: Side) -> u8 {
        let GroupPosition {
            inner_index,
            resting_order_index,
        } = self;

        // Resting orders always begin from left to right so the latter part is the same
        match side {
            Side::Bid => {
                (31 - inner_index.as_usize() as u8) * 8 + (resting_order_index.as_u8() + 1)
            }

            Side::Ask => (inner_index.as_usize() as u8) * 8 + (resting_order_index.as_u8() + 1),
        }
    }
}

impl From<&OrderId> for GroupPosition {
    fn from(value: &OrderId) -> Self {
        GroupPosition {
            inner_index: value.price_in_ticks.inner_index(),
            resting_order_index: value.resting_order_index,
        }
    }
}
