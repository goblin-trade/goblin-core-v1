use crate::state::Side;

use super::order_id::OrderId;

/// Utility struct to enforce sorting in order IDs
///
/// Consecutive orders must move away from centre
/// - Bids: Descending order
/// - Asks: Ascending order
///
/// For two orders at the same tick, tiebreak using the resting order index.
/// Orders with lower resting order position will be lower.
///
/// Two orders are sorted if `new_sorted_order_id > last_sorted_order_id`
/// for both `sides`.
/// The convention `new > last` follows the sort direction for asks (ascending order).
/// That is `new` will be the bigger value for asks and the smaller value for bids.
///
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct SortedOrderId {
    pub side: Side,
    pub order_id: OrderId,
}

impl PartialOrd for SortedOrderId {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for SortedOrderId {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        debug_assert_eq!(
            self.side, other.side,
            "Both SortedOrderId must belong to the same side"
        );

        match self.side {
            Side::Ask => self
                .order_id
                .price_in_ticks
                .cmp(&other.order_id.price_in_ticks)
                .then_with(|| {
                    self.order_id
                        .resting_order_index
                        .cmp(&other.order_id.resting_order_index)
                }),
            Side::Bid => other
                .order_id
                .price_in_ticks
                .cmp(&self.order_id.price_in_ticks) // Reverse for bids
                .then_with(|| {
                    self.order_id
                        .resting_order_index
                        .cmp(&other.order_id.resting_order_index)
                }),
        }
    }
}

/// Whether two order ids for a side are sorted
pub fn orders_are_sorted(side: Side, new_order_id: OrderId, last_order_id: OrderId) -> bool {
    let new_sorted_order_id = SortedOrderId {
        side,
        order_id: new_order_id,
    };

    let last_sorted_order_id = SortedOrderId {
        side,
        order_id: last_order_id,
    };

    new_sorted_order_id > last_sorted_order_id
}

#[cfg(test)]
mod tests {
    use crate::{
        quantities::{Ticks, WrapperU64},
        state::RestingOrderIndex,
    };

    use super::*;

    mod sorted_order_id {
        use super::*;

        #[test]
        fn test_ask_order_id_sorting() {
            // Asks in ascending order
            let ask1 = SortedOrderId {
                side: Side::Ask,
                order_id: OrderId {
                    price_in_ticks: Ticks::new(100),
                    resting_order_index: RestingOrderIndex::new(1),
                },
            };
            let ask2 = SortedOrderId {
                side: Side::Ask,
                order_id: OrderId {
                    price_in_ticks: Ticks::new(200),
                    resting_order_index: RestingOrderIndex::new(2),
                },
            };

            assert!(ask1 < ask2);

            let mut asks = vec![ask2, ask1];
            asks.sort();

            assert_eq!(asks, vec![ask1, ask2]);
        }

        #[test]
        fn test_bid_order_id_sorting() {
            let bid1 = SortedOrderId {
                side: Side::Bid,
                order_id: OrderId {
                    price_in_ticks: Ticks::new(100),
                    resting_order_index: RestingOrderIndex::ZERO,
                },
            };
            let bid2 = SortedOrderId {
                side: Side::Bid,
                order_id: OrderId {
                    price_in_ticks: Ticks::new(200),
                    resting_order_index: RestingOrderIndex::ZERO,
                },
            };

            assert!(bid1 > bid2);

            let mut bids = vec![bid1, bid2];
            bids.sort();

            assert_eq!(bids, vec![bid2, bid1]);
        }

        #[test]
        fn test_ask_order_id_resting_order_index_tiebreaker() {
            let ask1 = SortedOrderId {
                side: Side::Ask,
                order_id: OrderId {
                    price_in_ticks: Ticks::new(100),
                    resting_order_index: RestingOrderIndex::new(1),
                },
            };
            let ask2 = SortedOrderId {
                side: Side::Ask,
                order_id: OrderId {
                    price_in_ticks: Ticks::new(100),
                    resting_order_index: RestingOrderIndex::new(2),
                },
            };

            assert!(ask1 < ask2);

            let mut asks = vec![ask2, ask1];
            asks.sort();

            assert_eq!(asks, vec![ask1, ask2]);
        }

        #[test]
        fn test_bid_order_id_resting_order_index_tiebreaker() {
            let bid1 = SortedOrderId {
                side: Side::Bid,
                order_id: OrderId {
                    price_in_ticks: Ticks::new(200),
                    resting_order_index: RestingOrderIndex::new(1),
                },
            };
            let bid2 = SortedOrderId {
                side: Side::Bid,
                order_id: OrderId {
                    price_in_ticks: Ticks::new(200),
                    resting_order_index: RestingOrderIndex::new(2),
                },
            };

            assert!(bid1 < bid2);

            let mut bids = vec![bid2, bid1];
            bids.sort();

            assert_eq!(bids, vec![bid1, bid2]);
        }
    }

    mod orders_are_sorted {
        use super::*;

        #[test]
        fn bids_in_order() {
            let side = Side::Bid;

            // Descending order for bids- valid
            let last_order_id = OrderId {
                price_in_ticks: Ticks::new(2),
                resting_order_index: RestingOrderIndex::ZERO,
            };
            let new_order_id = OrderId {
                price_in_ticks: Ticks::new(1),
                resting_order_index: RestingOrderIndex::ZERO,
            };

            let sorted = orders_are_sorted(side, new_order_id, last_order_id);
            assert_eq!(sorted, true);
        }

        #[test]
        fn bids_not_in_order() {
            let side = Side::Bid;

            // Ascending order for bids- invalid
            let last_order_id = OrderId {
                price_in_ticks: Ticks::new(1),
                resting_order_index: RestingOrderIndex::ZERO,
            };
            let new_order_id = OrderId {
                price_in_ticks: Ticks::new(2),
                resting_order_index: RestingOrderIndex::ZERO,
            };

            let sorted = orders_are_sorted(side, new_order_id, last_order_id);
            assert_eq!(sorted, false);
        }

        #[test]
        fn asks_in_order() {
            let side = Side::Ask;

            // Ascending order for asks- valid
            let last_order_id = OrderId {
                price_in_ticks: Ticks::new(1),
                resting_order_index: RestingOrderIndex::ZERO,
            };
            let new_order_id = OrderId {
                price_in_ticks: Ticks::new(2),
                resting_order_index: RestingOrderIndex::ZERO,
            };

            let sorted = orders_are_sorted(side, new_order_id, last_order_id);
            assert_eq!(sorted, true);
        }

        #[test]
        fn asks_not_in_order() {
            let side = Side::Ask;

            // Descending order for asks- invalid
            let last_order_id = OrderId {
                price_in_ticks: Ticks::new(2),
                resting_order_index: RestingOrderIndex::ZERO,
            };
            let new_order_id = OrderId {
                price_in_ticks: Ticks::new(1),
                resting_order_index: RestingOrderIndex::ZERO,
            };

            let sorted = orders_are_sorted(side, new_order_id, last_order_id);
            assert_eq!(sorted, false);
        }
    }
}
