use crate::{
    quantities::{Ticks, WrapperU64},
    state::{MarketState, OuterIndex, RestingOrderIndex, Side, SlotKey, RESTING_ORDER_KEY_SEED},
};

use super::{group_position::GroupPosition, sorted_order_id::SortedOrderId};

#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct OrderId {
    /// Tick where order is placed
    pub price_in_ticks: Ticks,

    /// Resting order index between 0 to 7. A single tick can have at most 8 orders
    pub resting_order_index: RestingOrderIndex,
}

impl SlotKey for OrderId {
    fn get_key(&self) -> [u8; 32] {
        let mut key = [0u8; 32];

        key[0] = RESTING_ORDER_KEY_SEED;
        key[1..9].copy_from_slice(&self.price_in_ticks.as_u64().to_be_bytes());
        key[9] = self.resting_order_index.as_u8();

        key
    }
}

impl OrderId {
    pub fn decode(bytes: &[u8; 32]) -> Self {
        OrderId {
            price_in_ticks: Ticks::new(u64::from_be_bytes(bytes[1..9].try_into().unwrap())),
            resting_order_index: RestingOrderIndex::new(bytes[9]),
        }
    }

    /// Find the side of an active resting order (not a new order being placed)
    ///
    /// An active bid cannot have a price more than the best bid price,
    /// and an active ask cannot have a price lower than the best ask price.
    ///
    pub fn side(&self, market_state: &MarketState) -> Side {
        if self.price_in_ticks >= market_state.best_ask_price {
            Side::Ask
        } else if self.price_in_ticks <= market_state.best_bid_price {
            Side::Bid
        } else {
            // There are no active orders in the spread
            // However there could be activated slots. Ensure that they are not tested here.
            unreachable!()
        }
    }

    // TODO remove? I've already implemented from()
    pub fn from_group_position(group_position: GroupPosition, outer_index: OuterIndex) -> Self {
        OrderId {
            price_in_ticks: Ticks::from_indices(outer_index, group_position.inner_index),
            resting_order_index: group_position.resting_order_index,
        }
    }
}
