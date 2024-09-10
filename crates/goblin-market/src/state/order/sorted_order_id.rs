use super::order_id::OrderId;

// Asks are sorted in ascending order of price
#[derive(PartialEq, Eq, Debug, Clone, Copy, Ord, PartialOrd)]
pub struct AskOrderId {
    pub inner: OrderId,
}

// Bids are sorted in descending order of price
#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub struct BidOrderId {
    pub inner: OrderId,
}

impl PartialOrd for BidOrderId {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for BidOrderId {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        // Compare `Ticks` in descending order
        other
            .inner
            .price_in_ticks
            .cmp(&self.inner.price_in_ticks)
            .then_with(|| {
                self.inner
                    .resting_order_index
                    .cmp(&other.inner.resting_order_index)
            })
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        quantities::{Ticks, WrapperU64},
        state::RestingOrderIndex,
    };

    use super::*;

    #[test]
    fn test_ask_order_id_sorting() {
        let ask1 = AskOrderId {
            inner: OrderId {
                price_in_ticks: Ticks::new(100),
                resting_order_index: RestingOrderIndex::new(1),
            },
        };
        let ask2 = AskOrderId {
            inner: OrderId {
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
        let bid1 = BidOrderId {
            inner: OrderId {
                price_in_ticks: Ticks::new(100),
                resting_order_index: RestingOrderIndex::new(1),
            },
        };
        let bid2 = BidOrderId {
            inner: OrderId {
                price_in_ticks: Ticks::new(200),
                resting_order_index: RestingOrderIndex::new(2),
            },
        };

        assert!(bid2 < bid1);

        let mut bids = vec![bid1, bid2];
        bids.sort();

        assert_eq!(bids, vec![bid2, bid1]);
    }

    #[test]
    fn test_ask_order_id_resting_order_index_tiebreaker() {
        let ask1 = AskOrderId {
            inner: OrderId {
                price_in_ticks: Ticks::new(100),
                resting_order_index: RestingOrderIndex::new(1),
            },
        };
        let ask2 = AskOrderId {
            inner: OrderId {
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
        let bid1 = BidOrderId {
            inner: OrderId {
                price_in_ticks: Ticks::new(200),
                resting_order_index: RestingOrderIndex::new(1),
            },
        };
        let bid2 = BidOrderId {
            inner: OrderId {
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
