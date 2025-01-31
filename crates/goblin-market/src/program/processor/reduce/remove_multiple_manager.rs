use crate::{
    quantities::Ticks,
    state::{
        get_best_market_price,
        order::order_id::OrderId,
        remove::{IOrderLookupRemover, IOuterIndexLookupRemover, OrderLookupRemover},
        ArbContext, MarketState, Side,
    },
};

/// Manager to remove multiple orders in bulk for both sides
pub struct RemoveMultipleManager<'a> {
    pub side: Side,
    removers: [OrderLookupRemover<'a>; 2],
}

impl<'a> RemoveMultipleManager<'a> {
    pub fn new(
        best_bid_price: &'a mut Ticks,
        best_ask_price: &'a mut Ticks,
        bids_outer_indices: &'a mut u16,
        asks_outer_indices: &'a mut u16,
    ) -> Self {
        RemoveMultipleManager {
            side: Side::Bid,
            removers: [
                OrderLookupRemover::new(Side::Bid, best_bid_price, bids_outer_indices),
                OrderLookupRemover::new(Side::Ask, best_ask_price, asks_outer_indices),
            ],
        }
    }

    pub fn new_from_market(market_state: &'a mut MarketState) -> Self {
        RemoveMultipleManager::new(
            &mut market_state.best_bid_price,
            &mut market_state.best_ask_price,
            &mut market_state.bids_outer_indices,
            &mut market_state.asks_outer_indices,
        )
    }

    /// Check whether the given order ID is present in the book
    ///
    /// # Rules
    ///
    /// * order ids must be sorted such that their outer indices move away
    /// from the centre. I.e. outer indices of bids must be in descending order
    /// and for asks in ascending order.
    /// * inner indices can be random as long as outer ids are sorted, but it is
    /// recommended for sanity sake to have them sorted too.
    /// * Order ids for bids and asks can be randomly ordered. However gas can
    /// be optimized when both order ids are on the same outer index by grouping
    /// them by side to minimize transitioning between the removers and therefore reducing
    /// slot writes.
    ///
    /// # Arguments
    ///
    /// * `ctx`
    /// * `order_id` - Order ID to search
    pub fn find(&mut self, ctx: &mut ArbContext, order_id: OrderId) -> bool {
        if let Some(new_side) = self.get_side(order_id.price_in_ticks) {
            if new_side != self.side {
                let outer_index = order_id.price_in_ticks.outer_index();
                let old_remover = self.remover_mut(self.side);

                // Write the bitmap group if pending and share it with the opposite side remover.
                // If pending_write is false it is acceptable for bitmap groups in both removers
                // to drift.
                // old_remover.pending_write is false when sequential remover removes the outermost
                // order, changing the market price or closing the group. In these cases the
                // bitmap group is anyway not written, we simply update the best market price.
                //
                // By not sharing the group, the group actually becomes consistent with the
                // state that was supposed to written to slot by the old remover.
                if old_remover.pending_write && old_remover.outer_index() == Some(outer_index) {
                    old_remover.write_bitmap_group(ctx, outer_index);
                    let shared_bitmap_group = old_remover.get_shared_bitmap_group();

                    let new_remover = self.remover_mut(new_side);
                    new_remover.set_shared_bitmap_group(ctx, outer_index, shared_bitmap_group);
                }

                self.side = new_side;
            }

            self.current_remover_mut().find(ctx, order_id)
        } else {
            false
        }
    }

    /// Remove the last found order id from book.
    ///
    /// This function be called after calling find(). Trying to remove a None
    /// order id is a no-op
    pub fn remove(&mut self, ctx: &mut ArbContext) {
        self.current_remover_mut().remove(ctx)
    }

    /// Conclude the removals by commiting the last used remover
    pub fn commit(&mut self, ctx: &mut ArbContext) {
        self.remover_mut(self.side.opposite()).commit(ctx);
        self.remover_mut(self.side).commit(ctx);
    }

    // Getters

    fn remover(&self, side: Side) -> &OrderLookupRemover<'a> {
        &self.removers[side as usize]
    }

    fn remover_mut(&mut self, side: Side) -> &mut OrderLookupRemover<'a> {
        &mut self.removers[side as usize]
    }

    fn current_remover_mut(&mut self) -> &mut OrderLookupRemover<'a> {
        self.remover_mut(self.side)
    }

    /// Determine side for the order id being removed.
    ///
    /// The side for prices between best bid price and best ask price is indeterminate
    /// (None) since no removable orders exist here.
    fn get_side(&self, order_price: Ticks) -> Option<Side> {
        // Side is bid if price is equal to or futher from the centre than best bid price
        let best_bid_price = self.get_best_price_for_side(Side::Bid);
        if best_bid_price.is_some_and(|best_bid_price| order_price <= best_bid_price) {
            return Some(Side::Bid);
        }

        // Side is ask if price is equal to or futher from the centre than best ask price
        let best_ask_price = self.get_best_price_for_side(Side::Ask);
        if best_ask_price.is_some_and(|best_ask_price| order_price >= best_ask_price) {
            return Some(Side::Ask);
        }

        None
    }

    fn get_best_price_for_side(&self, side: Side) -> Option<Ticks> {
        let remover = self.remover(side);
        let market_price_inner = *remover.best_market_price;
        let outer_index_count = remover.outer_index_remover().total_outer_index_count();
        let best_price = get_best_market_price(market_price_inner, outer_index_count);

        best_price
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        quantities::WrapperU64,
        state::{
            bitmap_group::BitmapGroup, write_outer_indices_for_tests, ContextActions, InnerIndex,
            OuterIndex, RestingOrderIndex,
        },
    };

    use super::*;

    mod get_side {
        use super::*;

        #[test]
        fn test_get_side_for_price() {
            let best_bid_price = &mut Ticks::new(1);
            let best_ask_price = &mut Ticks::new(3);
            let bids_outer_indices = &mut 1;
            let asks_outer_indices = &mut 1;
            let manager = RemoveMultipleManager::new(
                best_bid_price,
                best_ask_price,
                bids_outer_indices,
                asks_outer_indices,
            );

            let order_price_0 = Ticks::new(1);
            assert_eq!(manager.get_side(order_price_0).unwrap(), Side::Bid);

            let order_price_1 = Ticks::new(2);
            assert_eq!(manager.get_side(order_price_1), None);

            let order_price_2 = Ticks::new(3);
            assert_eq!(manager.get_side(order_price_2).unwrap(), Side::Ask);
        }

        #[test]
        fn test_get_side_when_outer_index_count_is_zero() {
            let best_bid_price = &mut Ticks::new(1);
            let best_ask_price = &mut Ticks::new(3);
            let bids_outer_indices = &mut 0;
            let asks_outer_indices = &mut 0;
            let manager = RemoveMultipleManager::new(
                best_bid_price,
                best_ask_price,
                bids_outer_indices,
                asks_outer_indices,
            );

            let order_price_0 = Ticks::new(1);
            assert_eq!(manager.get_side(order_price_0), None);

            let order_price_1 = Ticks::new(2);
            assert_eq!(manager.get_side(order_price_1), None);

            let order_price_2 = Ticks::new(3);
            assert_eq!(manager.get_side(order_price_2), None);
        }
    }

    #[test]
    fn test_removing_bids_does_not_affect_ask_bits() {
        let ctx = &mut ArbContext::new();
        let outer_index_0 = OuterIndex::new(0);

        let mut bitmap_group_0 = BitmapGroup::default();
        bitmap_group_0.inner[2] = 0b0000_0001; // Best ask
        bitmap_group_0.inner[1] = 0b0000_0011; // Best bid
        bitmap_group_0.write_to_slot(ctx, &outer_index_0);

        write_outer_indices_for_tests(ctx, Side::Ask, vec![outer_index_0]);
        write_outer_indices_for_tests(ctx, Side::Bid, vec![outer_index_0]);

        let ask_order_id_0 = OrderId {
            price_in_ticks: Ticks::from_indices(outer_index_0, InnerIndex::new(2)),
            resting_order_index: RestingOrderIndex::new(0),
        };
        let bid_order_id_0 = OrderId {
            price_in_ticks: Ticks::from_indices(outer_index_0, InnerIndex::new(1)),
            resting_order_index: RestingOrderIndex::new(0),
        };
        let bid_order_id_1 = OrderId {
            price_in_ticks: Ticks::from_indices(outer_index_0, InnerIndex::new(1)),
            resting_order_index: RestingOrderIndex::new(1),
        };

        let mut best_bid_price = bid_order_id_0.price_in_ticks;
        let mut best_ask_price = ask_order_id_0.price_in_ticks;
        let mut bids_outer_indices = 1;
        let mut asks_outer_indices = 1;
        let mut manager = RemoveMultipleManager::new(
            &mut best_bid_price,
            &mut best_ask_price,
            &mut bids_outer_indices,
            &mut asks_outer_indices,
        );

        // to test whether side changed.
        // we can statically look at code to see that side changes when

        manager.find(ctx, bid_order_id_0);
        assert_eq!(manager.side, Side::Bid);
        manager.remove(ctx);
        manager.commit(ctx);
        drop(manager);

        let mut expected_bitmap_group_0 = BitmapGroup::default();
        expected_bitmap_group_0.inner[2] = 0b0000_0001; // Best ask
        expected_bitmap_group_0.inner[1] = 0b0000_0010; // Best bid

        // No change in bitmap group. Outer index is updated
        let read_bitmap_group_0 = BitmapGroup::new_from_slot(ctx, outer_index_0);
        assert_eq!(read_bitmap_group_0, expected_bitmap_group_0);

        assert_eq!(best_bid_price, bid_order_id_1.price_in_ticks);
        assert_eq!(best_ask_price, ask_order_id_0.price_in_ticks);
    }

    mod common_bitmap_group {
        use super::*;

        #[test]
        fn test_remove_from_same_group() {
            let ctx = &mut ArbContext::new();
            let outer_index_0 = OuterIndex::new(0);

            let mut bitmap_group_0 = BitmapGroup::default();
            bitmap_group_0.inner[2] = 0b0000_0011; // Best ask
            bitmap_group_0.inner[1] = 0b0000_0011; // Best bid
            bitmap_group_0.write_to_slot(ctx, &outer_index_0);

            write_outer_indices_for_tests(ctx, Side::Ask, vec![outer_index_0]);
            write_outer_indices_for_tests(ctx, Side::Bid, vec![outer_index_0]);

            let ask_order_id_0 = OrderId {
                price_in_ticks: Ticks::from_indices(outer_index_0, InnerIndex::new(2)),
                resting_order_index: RestingOrderIndex::new(0),
            };
            let bid_order_id_0 = OrderId {
                price_in_ticks: Ticks::from_indices(outer_index_0, InnerIndex::new(1)),
                resting_order_index: RestingOrderIndex::new(0),
            };

            let mut best_bid_price = bid_order_id_0.price_in_ticks;
            let mut best_ask_price = ask_order_id_0.price_in_ticks;
            let mut bids_outer_indices = 1;
            let mut asks_outer_indices = 1;
            let mut manager = RemoveMultipleManager::new(
                &mut best_bid_price,
                &mut best_ask_price,
                &mut bids_outer_indices,
                &mut asks_outer_indices,
            );

            // to test whether side changed.
            // we can statically look at code to see that side changes when

            manager.find(ctx, bid_order_id_0);
            assert_eq!(manager.side, Side::Bid);
            manager.remove(ctx);

            manager.find(ctx, ask_order_id_0);
            assert_eq!(manager.side, Side::Ask);
            manager.remove(ctx);

            let mut expected_bitmap_group_for_bid_remover = BitmapGroup::default();
            expected_bitmap_group_for_bid_remover.inner[2] = 0b0000_0011; // Best ask
            expected_bitmap_group_for_bid_remover.inner[1] = 0b0000_0010; // Best bid
            assert_eq!(
                manager
                    .remover(Side::Bid)
                    .group_position_remover
                    .active_group_position_iterator
                    .bitmap_group,
                expected_bitmap_group_for_bid_remover
            );

            let mut expected_bitmap_group_for_ask_remover = BitmapGroup::default();
            expected_bitmap_group_for_ask_remover.inner[2] = 0b0000_0010; // Best ask
            expected_bitmap_group_for_ask_remover.inner[1] = 0b0000_0010; // Best bid
            assert_eq!(
                manager
                    .remover(Side::Ask)
                    .group_position_remover
                    .active_group_position_iterator
                    .bitmap_group,
                expected_bitmap_group_for_ask_remover
            );

            manager.commit(ctx);

            let read_bitmap_group = BitmapGroup::new_from_slot(ctx, outer_index_0);
            assert_eq!(read_bitmap_group, expected_bitmap_group_for_ask_remover);
        }

        #[test]
        fn test_remove_from_same_group_then_move_to_next() {
            let ctx = &mut ArbContext::new();
            let outer_index_0 = OuterIndex::new(0);
            let outer_index_1 = OuterIndex::new(1);

            write_outer_indices_for_tests(ctx, Side::Ask, vec![outer_index_1, outer_index_0]);
            write_outer_indices_for_tests(ctx, Side::Bid, vec![outer_index_0]);

            let mut bitmap_group_1 = BitmapGroup::default();
            bitmap_group_1.inner[0] = 0b0000_0011;
            bitmap_group_1.write_to_slot(ctx, &outer_index_1);

            let mut bitmap_group_0 = BitmapGroup::default();
            bitmap_group_0.inner[2] = 0b0000_0001; // Best ask
            bitmap_group_0.inner[1] = 0b0000_0011; // Best bid
            bitmap_group_0.write_to_slot(ctx, &outer_index_0);

            let ask_order_id_2 = OrderId {
                price_in_ticks: Ticks::from_indices(outer_index_1, InnerIndex::new(0)),
                resting_order_index: RestingOrderIndex::new(1),
            };
            let ask_order_id_1 = OrderId {
                price_in_ticks: Ticks::from_indices(outer_index_1, InnerIndex::new(0)),
                resting_order_index: RestingOrderIndex::new(0),
            };
            let ask_order_id_0 = OrderId {
                price_in_ticks: Ticks::from_indices(outer_index_0, InnerIndex::new(2)),
                resting_order_index: RestingOrderIndex::new(0),
            };
            let bid_order_id_0 = OrderId {
                price_in_ticks: Ticks::from_indices(outer_index_0, InnerIndex::new(1)),
                resting_order_index: RestingOrderIndex::new(0),
            };
            let bid_order_id_1 = OrderId {
                price_in_ticks: Ticks::from_indices(outer_index_0, InnerIndex::new(1)),
                resting_order_index: RestingOrderIndex::new(1),
            };

            let mut best_bid_price = bid_order_id_0.price_in_ticks;
            let mut best_ask_price = ask_order_id_0.price_in_ticks;
            let mut bids_outer_indices = 1;
            let mut asks_outer_indices = 2;
            let mut manager = RemoveMultipleManager::new(
                &mut best_bid_price,
                &mut best_ask_price,
                &mut bids_outer_indices,
                &mut asks_outer_indices,
            );

            // to test whether side changed.
            // we can statically look at code to see that side changes when

            manager.find(ctx, bid_order_id_0);
            manager.remove(ctx);

            manager.find(ctx, ask_order_id_0);
            manager.remove(ctx);

            manager.find(ctx, ask_order_id_1);
            manager.remove(ctx);

            manager.commit(ctx);

            assert_eq!(
                manager.get_best_price_for_side(Side::Ask).unwrap(),
                ask_order_id_2.price_in_ticks
            );
            assert_eq!(
                manager.get_best_price_for_side(Side::Bid).unwrap(),
                bid_order_id_1.price_in_ticks
            );

            let mut expected_bitmap_group_0 = BitmapGroup::default();
            expected_bitmap_group_0.inner[2] = 0b0000_0001; // Garbage bit from closed ask
            expected_bitmap_group_0.inner[1] = 0b0000_0010; // Best bid
            let read_bitmap_group_0 = BitmapGroup::new_from_slot(ctx, outer_index_0);
            assert_eq!(read_bitmap_group_0, expected_bitmap_group_0);

            let mut expected_bitmap_group_1 = BitmapGroup::default();
            expected_bitmap_group_1.inner[0] = 0b0000_0010; // Best ask
            let read_bitmap_group_1 = BitmapGroup::new_from_slot(ctx, outer_index_1);
            assert_eq!(read_bitmap_group_1, expected_bitmap_group_1);
        }

        #[test]
        fn test_remove_out_of_order_from_same_group() {
            let ctx = &mut ArbContext::new();
            let outer_index_0 = OuterIndex::new(0);

            let mut bitmap_group_0 = BitmapGroup::default();
            bitmap_group_0.inner[2] = 0b0000_0001;
            bitmap_group_0.inner[1] = 0b0000_0001; // Best ask
            bitmap_group_0.inner[0] = 0b0000_0011; // Best bid
            bitmap_group_0.write_to_slot(ctx, &outer_index_0);

            write_outer_indices_for_tests(ctx, Side::Ask, vec![outer_index_0]);
            write_outer_indices_for_tests(ctx, Side::Bid, vec![outer_index_0]);

            // This was supposed to be a bid when trader placed order. Instead it now
            // holds an ask placed by another trader.
            let ask_order_id_0 = OrderId {
                price_in_ticks: Ticks::from_indices(outer_index_0, InnerIndex::new(1)),
                resting_order_index: RestingOrderIndex::new(0),
            };
            let ask_order_id_1 = OrderId {
                price_in_ticks: Ticks::from_indices(outer_index_0, InnerIndex::new(2)),
                resting_order_index: RestingOrderIndex::new(0),
            };

            let bid_order_id_0 = OrderId {
                price_in_ticks: Ticks::from_indices(outer_index_0, InnerIndex::new(0)),
                resting_order_index: RestingOrderIndex::new(0),
            };

            let mut best_bid_price = bid_order_id_0.price_in_ticks;
            let mut best_ask_price = ask_order_id_0.price_in_ticks;
            let mut bids_outer_indices = 1;
            let mut asks_outer_indices = 1;
            let mut manager = RemoveMultipleManager::new(
                &mut best_bid_price,
                &mut best_ask_price,
                &mut bids_outer_indices,
                &mut asks_outer_indices,
            );

            // This was believed to be a bid
            assert!(manager.find(ctx, ask_order_id_0));
            assert_eq!(manager.side, Side::Ask);

            // Remove bid
            assert!(manager.find(ctx, bid_order_id_0));
            assert_eq!(manager.side, Side::Bid);
            manager.remove(ctx);

            // Back to asks
            assert!(manager.find(ctx, ask_order_id_1));
            assert_eq!(manager.side, Side::Ask);
            manager.remove(ctx);

            let mut expected_bitmap_group_for_bid_remover = BitmapGroup::default();
            expected_bitmap_group_for_bid_remover.inner[2] = 0b0000_0001;
            expected_bitmap_group_for_bid_remover.inner[1] = 0b0000_0001; // Best ask
            expected_bitmap_group_for_bid_remover.inner[0] = 0b0000_0010; // Best bid
            assert_eq!(
                manager
                    .remover(Side::Bid)
                    .group_position_remover
                    .active_group_position_iterator
                    .bitmap_group,
                expected_bitmap_group_for_bid_remover
            );

            let mut expected_bitmap_group_for_ask_remover = BitmapGroup::default();
            expected_bitmap_group_for_ask_remover.inner[1] = 0b0000_0001; // Best ask
            expected_bitmap_group_for_ask_remover.inner[0] = 0b0000_0010; // Best bid
            assert_eq!(
                manager
                    .remover(Side::Ask)
                    .group_position_remover
                    .active_group_position_iterator
                    .bitmap_group,
                expected_bitmap_group_for_ask_remover
            );

            manager.commit(ctx);

            let read_bitmap_group = BitmapGroup::new_from_slot(ctx, outer_index_0);
            assert_eq!(read_bitmap_group, expected_bitmap_group_for_ask_remover);
        }

        #[test]
        fn test_remove_out_of_order_from_same_group_with_group_closing_for_bids() {
            let ctx = &mut ArbContext::new();
            let outer_index_0 = OuterIndex::new(0);

            let mut bitmap_group_0 = BitmapGroup::default();
            bitmap_group_0.inner[2] = 0b0000_0001;
            bitmap_group_0.inner[1] = 0b0000_0001; // Best ask
            bitmap_group_0.inner[0] = 0b0000_0001; // Best bid
            bitmap_group_0.write_to_slot(ctx, &outer_index_0);

            write_outer_indices_for_tests(ctx, Side::Ask, vec![outer_index_0]);
            write_outer_indices_for_tests(ctx, Side::Bid, vec![outer_index_0]);

            // This was supposed to be a bid when trader placed order. Instead it now
            // holds an ask placed by another trader.
            let ask_order_id_0 = OrderId {
                price_in_ticks: Ticks::from_indices(outer_index_0, InnerIndex::new(1)),
                resting_order_index: RestingOrderIndex::new(0),
            };
            let ask_order_id_1 = OrderId {
                price_in_ticks: Ticks::from_indices(outer_index_0, InnerIndex::new(2)),
                resting_order_index: RestingOrderIndex::new(0),
            };

            // Removing the bid causes group to close
            let bid_order_id_0 = OrderId {
                price_in_ticks: Ticks::from_indices(outer_index_0, InnerIndex::new(0)),
                resting_order_index: RestingOrderIndex::new(0),
            };

            let mut best_bid_price = bid_order_id_0.price_in_ticks;
            let mut best_ask_price = ask_order_id_0.price_in_ticks;
            let mut bids_outer_indices = 1;
            let mut asks_outer_indices = 1;
            let mut manager = RemoveMultipleManager::new(
                &mut best_bid_price,
                &mut best_ask_price,
                &mut bids_outer_indices,
                &mut asks_outer_indices,
            );

            // This was believed to be a bid
            assert!(manager.find(ctx, ask_order_id_0));
            assert_eq!(manager.side, Side::Ask);

            // Remove bid
            assert!(manager.find(ctx, bid_order_id_0));
            assert_eq!(manager.side, Side::Bid);
            manager.remove(ctx);

            // Back to asks
            assert!(manager.find(ctx, ask_order_id_1));
            assert_eq!(manager.side, Side::Ask);
            manager.remove(ctx);

            // Since the last bit was removed by the sequential remover, this bitmap
            // group is not written.
            let mut expected_bitmap_group_for_bid_remover = BitmapGroup::default();
            expected_bitmap_group_for_bid_remover.inner[2] = 0b0000_0001;
            expected_bitmap_group_for_bid_remover.inner[1] = 0b0000_0001; // Best ask
            assert_eq!(
                manager
                    .remover(Side::Bid)
                    .group_position_remover
                    .active_group_position_iterator
                    .bitmap_group,
                expected_bitmap_group_for_bid_remover
            );

            // Since pending_write was false the bitmap group for bids was not written to slot.
            // The closed order at bid_order_id_0 becomes a garbage bit
            let mut expected_bitmap_group_for_ask_remover = BitmapGroup::default();
            expected_bitmap_group_for_ask_remover.inner[1] = 0b0000_0001; // Best ask
            expected_bitmap_group_for_ask_remover.inner[0] = 0b0000_0001; // Garbage bit from closed bid
            assert_eq!(
                manager
                    .remover(Side::Ask)
                    .group_position_remover
                    .active_group_position_iterator
                    .bitmap_group,
                expected_bitmap_group_for_ask_remover
            );

            manager.commit(ctx);

            let read_bitmap_group = BitmapGroup::new_from_slot(ctx, outer_index_0);
            assert_eq!(read_bitmap_group, expected_bitmap_group_for_ask_remover);
        }

        #[test]
        fn test_remove_out_of_order_from_same_group_with_group_closing_for_asks() {
            let ctx = &mut ArbContext::new();
            let outer_index_0 = OuterIndex::new(0);

            let mut bitmap_group_0 = BitmapGroup::default();
            bitmap_group_0.inner[2] = 0b0000_0001; // Best ask
            bitmap_group_0.inner[1] = 0b0000_0001; // Best bid
            bitmap_group_0.inner[0] = 0b0000_0001;
            bitmap_group_0.write_to_slot(ctx, &outer_index_0);

            write_outer_indices_for_tests(ctx, Side::Ask, vec![outer_index_0]);
            write_outer_indices_for_tests(ctx, Side::Bid, vec![outer_index_0]);

            // This was supposed to be an ask when trader placed order. Instead it now
            // holds a bid placed by another trader.
            let bid_order_id_0 = OrderId {
                price_in_ticks: Ticks::from_indices(outer_index_0, InnerIndex::new(1)),
                resting_order_index: RestingOrderIndex::new(0),
            };

            let bid_order_id_1 = OrderId {
                price_in_ticks: Ticks::from_indices(outer_index_0, InnerIndex::new(0)),
                resting_order_index: RestingOrderIndex::new(0),
            };

            let ask_order_id_0 = OrderId {
                price_in_ticks: Ticks::from_indices(outer_index_0, InnerIndex::new(2)),
                resting_order_index: RestingOrderIndex::new(0),
            };

            let mut best_bid_price = bid_order_id_0.price_in_ticks;
            let mut best_ask_price = ask_order_id_0.price_in_ticks;
            let mut bids_outer_indices = 1;
            let mut asks_outer_indices = 1;
            let mut manager = RemoveMultipleManager::new(
                &mut best_bid_price,
                &mut best_ask_price,
                &mut bids_outer_indices,
                &mut asks_outer_indices,
            );

            // This was believed to be an ask
            assert!(manager.find(ctx, bid_order_id_0));
            assert_eq!(manager.side, Side::Bid);

            // Remove ask
            assert!(manager.find(ctx, ask_order_id_0));
            assert_eq!(manager.side, Side::Ask);
            manager.remove(ctx);

            // Back to bids
            assert!(manager.find(ctx, bid_order_id_1));
            assert_eq!(manager.side, Side::Bid);
            manager.remove(ctx);

            // Since the last bit was removed by the sequential remover, this bitmap
            // group is not written.
            let mut expected_bitmap_group_for_ask_remover = BitmapGroup::default();
            expected_bitmap_group_for_ask_remover.inner[1] = 0b0000_0001; // Best bid
            expected_bitmap_group_for_ask_remover.inner[0] = 0b0000_0001;
            assert_eq!(
                manager
                    .remover(Side::Ask)
                    .group_position_remover
                    .active_group_position_iterator
                    .bitmap_group,
                expected_bitmap_group_for_ask_remover
            );

            // Since pending_write was false the bitmap group for asks was not written to slot.
            // The closed order at ask_order_id_0 becomes a garbage bit
            let mut expected_bitmap_group_for_bid_remover = BitmapGroup::default();
            expected_bitmap_group_for_bid_remover.inner[2] = 0b0000_0001; // Garbage bit from closed ask
            expected_bitmap_group_for_bid_remover.inner[1] = 0b0000_0001; // Best bid
            assert_eq!(
                manager
                    .remover(Side::Bid)
                    .group_position_remover
                    .active_group_position_iterator
                    .bitmap_group,
                expected_bitmap_group_for_bid_remover
            );

            manager.commit(ctx);

            let read_bitmap_group = BitmapGroup::new_from_slot(ctx, outer_index_0);
            assert_eq!(read_bitmap_group, expected_bitmap_group_for_bid_remover);
        }

        #[test]
        fn test_remove_out_of_order_from_same_group_with_price_changing() {
            let ctx = &mut ArbContext::new();
            let outer_index_0 = OuterIndex::new(0);

            let mut bitmap_group_0 = BitmapGroup::default();
            bitmap_group_0.inner[3] = 0b0000_0001;
            bitmap_group_0.inner[2] = 0b0000_0001; // Best ask
            bitmap_group_0.inner[1] = 0b0000_0001; // Best bid
            bitmap_group_0.inner[0] = 0b0000_0001;
            bitmap_group_0.write_to_slot(ctx, &outer_index_0);

            write_outer_indices_for_tests(ctx, Side::Ask, vec![outer_index_0]);
            write_outer_indices_for_tests(ctx, Side::Bid, vec![outer_index_0]);

            // This was supposed to be a bid when trader placed order. Instead it now
            // holds an ask placed by another trader.
            let ask_order_id_0 = OrderId {
                price_in_ticks: Ticks::from_indices(outer_index_0, InnerIndex::new(2)),
                resting_order_index: RestingOrderIndex::new(0),
            };
            let ask_order_id_1 = OrderId {
                price_in_ticks: Ticks::from_indices(outer_index_0, InnerIndex::new(3)),
                resting_order_index: RestingOrderIndex::new(0),
            };

            // Removing the bid causes best price to update
            let bid_order_id_0 = OrderId {
                price_in_ticks: Ticks::from_indices(outer_index_0, InnerIndex::new(1)),
                resting_order_index: RestingOrderIndex::new(0),
            };

            let mut best_bid_price = bid_order_id_0.price_in_ticks;
            let mut best_ask_price = ask_order_id_0.price_in_ticks;
            let mut bids_outer_indices = 1;
            let mut asks_outer_indices = 1;
            let mut manager = RemoveMultipleManager::new(
                &mut best_bid_price,
                &mut best_ask_price,
                &mut bids_outer_indices,
                &mut asks_outer_indices,
            );

            // This was believed to be a bid
            assert!(manager.find(ctx, ask_order_id_0));
            assert_eq!(manager.side, Side::Ask);

            // Remove bid
            assert!(manager.find(ctx, bid_order_id_0));
            assert_eq!(manager.side, Side::Bid);
            manager.remove(ctx);

            // Back to asks
            assert!(manager.find(ctx, ask_order_id_1));
            assert_eq!(manager.side, Side::Ask);
            manager.remove(ctx);

            // Since best price updates, pending write is false. This group is not written.
            let mut expected_bitmap_group_for_bid_remover = BitmapGroup::default();
            expected_bitmap_group_for_bid_remover.inner[3] = 0b0000_0001;
            expected_bitmap_group_for_bid_remover.inner[2] = 0b0000_0001; // Best ask
            expected_bitmap_group_for_bid_remover.inner[0] = 0b0000_0001; // Best bid
            assert_eq!(
                manager
                    .remover(Side::Bid)
                    .group_position_remover
                    .active_group_position_iterator
                    .bitmap_group,
                expected_bitmap_group_for_bid_remover
            );

            let mut expected_bitmap_group_for_ask_remover = BitmapGroup::default();
            expected_bitmap_group_for_ask_remover.inner[2] = 0b0000_0001; // Best ask
            expected_bitmap_group_for_ask_remover.inner[1] = 0b0000_0001; // Garbage bit from closed bid
            expected_bitmap_group_for_ask_remover.inner[0] = 0b0000_0001; // Best bid
            assert_eq!(
                manager
                    .remover(Side::Ask)
                    .group_position_remover
                    .active_group_position_iterator
                    .bitmap_group,
                expected_bitmap_group_for_ask_remover
            );

            manager.commit(ctx);

            let read_bitmap_group = BitmapGroup::new_from_slot(ctx, outer_index_0);
            assert_eq!(read_bitmap_group, expected_bitmap_group_for_ask_remover);
        }
    }
}
