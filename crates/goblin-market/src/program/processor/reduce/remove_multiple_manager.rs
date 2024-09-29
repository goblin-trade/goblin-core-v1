use crate::{
    program::{GoblinError, GoblinResult, PricesNotInOrder},
    require,
    state::{
        order::{order_id::OrderId, sorted_order_id::orders_are_sorted},
        remove::order_id_remover::OrderIdRemover,
        ArbContext, MarketState, Side,
    },
};

/// Boilerplate code to remove multiple orders in bulk for both sides
pub struct RemoveMultipleManager {
    side: Side,
    last_order_ids: [Option<OrderId>; 2],
    removers: [OrderIdRemover; 2],
}

impl RemoveMultipleManager {
    pub fn new(bids_outer_indices: u16, asks_outer_indices: u16) -> Self {
        RemoveMultipleManager {
            side: Side::Bid,
            last_order_ids: [None, None],
            removers: [
                OrderIdRemover::new(bids_outer_indices, Side::Bid),
                OrderIdRemover::new(asks_outer_indices, Side::Ask),
            ],
        }
    }

    fn remover(&mut self) -> &mut OrderIdRemover {
        &mut self.removers[self.side as usize]
    }

    fn last_order_id(&mut self) -> &mut Option<OrderId> {
        &mut self.last_order_ids[self.side as usize]
    }

    /// Checks whether an order is present at the given order ID.
    pub fn find_order(
        &mut self,
        ctx: &mut ArbContext,
        side: Side,
        order_id: OrderId,
    ) -> GoblinResult<bool> {
        self.check_sorted(side, order_id)?;

        let found = self.remover().order_id_is_active(ctx, order_id);
        Ok(found)
    }

    /// Ensures that successive order ids to remove are sorted in correct order
    ///
    /// Successive IDs must be in ascending order for asks and in descending order for bids
    ///
    /// Externally ensure that `remove_order()` is not called if incoming order is not in
    /// correct order to avoid duplicate removal. This function updates `self.side`
    /// even though the incoming order is not added to state.
    ///
    pub(crate) fn check_sorted(&mut self, side: Side, order_id: OrderId) -> GoblinResult<()> {
        self.side = side;
        let last_order_id = self.last_order_id();

        // Successive orders must move away from the centre
        if let Some(last_order_id) = *last_order_id {
            let sorted = orders_are_sorted(side, order_id, last_order_id);
            require!(sorted, GoblinError::PricesNotInOrder(PricesNotInOrder {}));
        }
        // Set as last order ID
        *last_order_id = Some(order_id);

        Ok(())
    }

    /// Remove the last searched order from the book, and update the
    /// best price in market state if the outermost tick closed
    pub fn remove_order(&mut self, ctx: &mut ArbContext, market_state: &mut MarketState) {
        self.remover().remove_order(ctx, market_state)
    }

    /// Write the prepared outer indices to slot and update outer index count in market state
    /// The last cached bitmap group pending a write is also written to slot
    pub fn write_prepared_indices(&mut self, ctx: &mut ArbContext, market_state: &mut MarketState) {
        self.removers[0].write_prepared_indices(ctx, market_state);
        self.removers[1].write_prepared_indices(ctx, market_state);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod check_sorted {
        use crate::{
            quantities::{Ticks, WrapperU64},
            state::RestingOrderIndex,
        };

        use super::*;

        #[test]
        pub fn test_bid_order_sequence_enforced() {
            let side = Side::Bid;

            let mut manager = RemoveMultipleManager::new(0, 0);

            // First value
            let order_id_0 = OrderId {
                price_in_ticks: Ticks::new(4),
                resting_order_index: RestingOrderIndex::ZERO,
            };
            manager.check_sorted(side, order_id_0).unwrap();
            assert_eq!((*manager.last_order_id()).unwrap(), order_id_0);

            // Second- overwrites first
            let order_id_1 = OrderId {
                price_in_ticks: Ticks::new(3),
                resting_order_index: RestingOrderIndex::ZERO,
            };
            manager.check_sorted(side, order_id_1).unwrap();
            assert_eq!((*manager.last_order_id()).unwrap(), order_id_1);

            // Third- not in order so fail
            let order_id_2 = OrderId {
                price_in_ticks: Ticks::new(5),
                resting_order_index: RestingOrderIndex::ZERO,
            };
            assert!(manager.check_sorted(side, order_id_2).is_err());
            assert_eq!((*manager.last_order_id()).unwrap(), order_id_1);

            // Fourth- higher resting order index
            let order_id_3 = OrderId {
                price_in_ticks: Ticks::new(3),
                resting_order_index: RestingOrderIndex::MAX,
            };
            manager.check_sorted(side, order_id_3).unwrap();
            assert_eq!((*manager.last_order_id()).unwrap(), order_id_3);

            // Fifth- same tick but lower resting order index so fail
            let order_id_4 = OrderId {
                price_in_ticks: Ticks::new(3),
                resting_order_index: RestingOrderIndex::new(1),
            };
            assert!(manager.check_sorted(side, order_id_4).is_err());
            assert_eq!((*manager.last_order_id()).unwrap(), order_id_3);
        }

        #[test]
        pub fn test_ask_order_sequence_enforced() {
            let side = Side::Ask;
            let mut manager = RemoveMultipleManager::new(0, 0);

            // First value
            let order_id_0 = OrderId {
                price_in_ticks: Ticks::new(4),
                resting_order_index: RestingOrderIndex::ZERO,
            };
            manager.check_sorted(side, order_id_0).unwrap();
            assert_eq!((*manager.last_order_id()).unwrap(), order_id_0);

            // Second- overwrites first
            let order_id_1 = OrderId {
                price_in_ticks: Ticks::new(5),
                resting_order_index: RestingOrderIndex::ZERO,
            };
            manager.check_sorted(side, order_id_1).unwrap();
            assert_eq!((*manager.last_order_id()).unwrap(), order_id_1);

            // Third- not in order so fail
            let order_id_2 = OrderId {
                price_in_ticks: Ticks::new(3),
                resting_order_index: RestingOrderIndex::ZERO,
            };
            assert!(manager.check_sorted(side, order_id_2).is_err());
            assert_eq!((*manager.last_order_id()).unwrap(), order_id_1);

            // Fourth- higher resting order index
            let order_id_3 = OrderId {
                price_in_ticks: Ticks::new(5),
                resting_order_index: RestingOrderIndex::MAX,
            };
            manager.check_sorted(side, order_id_3).unwrap();
            assert_eq!((*manager.last_order_id()).unwrap(), order_id_3);

            // Fifth- same tick but lower resting order index so fail
            let order_id_4 = OrderId {
                price_in_ticks: Ticks::new(5),
                resting_order_index: RestingOrderIndex::new(1),
            };
            assert!(manager.check_sorted(side, order_id_4).is_err());
            assert_eq!((*manager.last_order_id()).unwrap(), order_id_3);
        }

        #[test]
        pub fn test_bids_and_asks_mixed_but_respective_sequence_maintained() {
            let mut manager = RemoveMultipleManager::new(0, 0);

            // Insert bid, ask, bid, ask in correct sequence,
            // then add bid, ask in wrong sequence

            // 0. Bid
            let order_id_0 = OrderId {
                price_in_ticks: Ticks::new(4),
                resting_order_index: RestingOrderIndex::ZERO,
            };
            manager.check_sorted(Side::Bid, order_id_0).unwrap();
            assert_eq!((*manager.last_order_id()).unwrap(), order_id_0);
            assert_eq!(manager.side, Side::Bid);

            // 1. Ask
            let order_id_1 = OrderId {
                price_in_ticks: Ticks::new(7),
                resting_order_index: RestingOrderIndex::ZERO,
            };
            manager.check_sorted(Side::Ask, order_id_1).unwrap();
            assert_eq!((*manager.last_order_id()).unwrap(), order_id_1);
            assert_eq!(manager.side, Side::Ask);

            // 2. Bid
            let order_id_2 = OrderId {
                price_in_ticks: Ticks::new(3),
                resting_order_index: RestingOrderIndex::ZERO,
            };
            manager.check_sorted(Side::Bid, order_id_2).unwrap();
            assert_eq!((*manager.last_order_id()).unwrap(), order_id_2);
            assert_eq!(manager.side, Side::Bid);

            // 3. Ask
            let order_id_3 = OrderId {
                price_in_ticks: Ticks::new(8),
                resting_order_index: RestingOrderIndex::ZERO,
            };
            manager.check_sorted(Side::Ask, order_id_3).unwrap();
            assert_eq!((*manager.last_order_id()).unwrap(), order_id_3);
            assert_eq!(manager.side, Side::Ask);

            // 4. Out of order bid
            let order_id_4 = OrderId {
                price_in_ticks: Ticks::new(5),
                resting_order_index: RestingOrderIndex::ZERO,
            };

            assert!(manager.check_sorted(Side::Bid, order_id_4).is_err());

            // Since side changed, last_order_id will change
            assert_eq!(manager.side, Side::Bid);
            assert_eq!((*manager.last_order_id()).unwrap(), order_id_2);

            // 5. Out of order ask
            let order_id_5 = OrderId {
                price_in_ticks: Ticks::new(6),
                resting_order_index: RestingOrderIndex::ZERO,
            };
            // Last order ID did not change but side changed
            assert!(manager.check_sorted(Side::Ask, order_id_5).is_err());

            // Since side changed, last_order_id will change
            assert_eq!(manager.side, Side::Ask);
            assert_eq!((*manager.last_order_id()).unwrap(), order_id_3);
        }
    }
}
