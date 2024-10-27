use crate::{
    quantities::Ticks,
    state::{
        get_best_market_price,
        order::order_id::OrderId,
        remove::{
            IOrderLookupRemover, IOrderLookupRemoverInner, IOuterIndexLookupRemover,
            OrderLookupRemover,
        },
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
                OrderLookupRemover::new(Side::Bid, best_ask_price, asks_outer_indices),
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
    /// * Bids must be searched first, then asks. If you try to search for a bid
    /// when self.side is Ask, the result will always be false.
    /// * order ids must be sorted such that their outer indices move away
    /// from the centre. I.e. outer indices of bids must be in descending order
    /// and for asks in ascending order.
    /// * inner indices can be random as long as outer ids are sorted, but it is
    /// recommended for sanity sake to have them sorted too.
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

                if old_remover.outer_index() == Some(outer_index) && !old_remover.pending_read {
                    // Bitmap group has already been read by old remover
                    // Copy over the bitmap group and set pending read to false there
                    // Have a find_with_known_group() to the remover which accepts
                    // the bitmap group.
                    // What if the new remover uses sequential remover and moves to the next
                    // group? In this case the bitmap group is not supposed to be written.
                }

                if old_remover.pending_write && old_remover.outer_index() == Some(outer_index) {
                    // write old remover. pending_write toggle rules don't account for opposite
                    // side orders so we must write the group.
                    // Avoid committing old remover, as it could be needed again. When writing
                    // the common bitmap, we should see if pending_write in either of the removers
                    // is true.
                    // TODO old remover should remain usable after commiting.
                    // TODO commit should
                    old_remover.commit(ctx);

                    // pass bitmap group to new remover
                }
            }

            if self.side == Side::Ask && new_side == Side::Bid {
                // Cannot search for bids after asks were searched
                return false;
            } else if self.side == Side::Bid && new_side == Side::Ask {
                self.current_remover_mut().commit(ctx);
                self.side = Side::Ask;
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
        self.current_remover_mut().commit(ctx);
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

    // tests
    // - ask and bid order ids in jumbled order
    // - best ask price and best bid price are on same group. Removal of bids shouldn't
    // rewrite bits belonging to asks

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

    // problem- both removers hold their own copies of bitmap group. What
    // if the second commit overwrites the group written in the first commit?
    // We cannot have a common share by reference because different outer
    // indices will be traversed.
    #[test]
    fn test_removers_do_not_overwrite_bitmap_group() {
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

    // scenarios
    // bids first
    // asks first
    // mixed up bids and asks
    mod ensure_sort_order {}
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     mod check_sorted {
//         use crate::{
//             quantities::{Ticks, WrapperU64},
//             state::RestingOrderIndex,
//         };

//         use super::*;

//         #[test]
//         pub fn test_bid_order_sequence_enforced() {
//             let side = Side::Bid;

//             let mut manager = RemoveMultipleManager::new(0, 0);

//             // First value
//             let order_id_0 = OrderId {
//                 price_in_ticks: Ticks::new(4),
//                 resting_order_index: RestingOrderIndex::ZERO,
//             };
//             manager.check_sorted(side, order_id_0).unwrap();
//             assert_eq!((*manager.last_order_id()).unwrap(), order_id_0);

//             // Second- overwrites first
//             let order_id_1 = OrderId {
//                 price_in_ticks: Ticks::new(3),
//                 resting_order_index: RestingOrderIndex::ZERO,
//             };
//             manager.check_sorted(side, order_id_1).unwrap();
//             assert_eq!((*manager.last_order_id()).unwrap(), order_id_1);

//             // Third- not in order so fail
//             let order_id_2 = OrderId {
//                 price_in_ticks: Ticks::new(5),
//                 resting_order_index: RestingOrderIndex::ZERO,
//             };
//             assert!(manager.check_sorted(side, order_id_2).is_err());
//             assert_eq!((*manager.last_order_id()).unwrap(), order_id_1);

//             // Fourth- higher resting order index
//             let order_id_3 = OrderId {
//                 price_in_ticks: Ticks::new(3),
//                 resting_order_index: RestingOrderIndex::MAX,
//             };
//             manager.check_sorted(side, order_id_3).unwrap();
//             assert_eq!((*manager.last_order_id()).unwrap(), order_id_3);

//             // Fifth- same tick but lower resting order index so fail
//             let order_id_4 = OrderId {
//                 price_in_ticks: Ticks::new(3),
//                 resting_order_index: RestingOrderIndex::new(1),
//             };
//             assert!(manager.check_sorted(side, order_id_4).is_err());
//             assert_eq!((*manager.last_order_id()).unwrap(), order_id_3);
//         }

//         #[test]
//         pub fn test_ask_order_sequence_enforced() {
//             let side = Side::Ask;
//             let mut manager = RemoveMultipleManager::new(0, 0);

//             // First value
//             let order_id_0 = OrderId {
//                 price_in_ticks: Ticks::new(4),
//                 resting_order_index: RestingOrderIndex::ZERO,
//             };
//             manager.check_sorted(side, order_id_0).unwrap();
//             assert_eq!((*manager.last_order_id()).unwrap(), order_id_0);

//             // Second- overwrites first
//             let order_id_1 = OrderId {
//                 price_in_ticks: Ticks::new(5),
//                 resting_order_index: RestingOrderIndex::ZERO,
//             };
//             manager.check_sorted(side, order_id_1).unwrap();
//             assert_eq!((*manager.last_order_id()).unwrap(), order_id_1);

//             // Third- not in order so fail
//             let order_id_2 = OrderId {
//                 price_in_ticks: Ticks::new(3),
//                 resting_order_index: RestingOrderIndex::ZERO,
//             };
//             assert!(manager.check_sorted(side, order_id_2).is_err());
//             assert_eq!((*manager.last_order_id()).unwrap(), order_id_1);

//             // Fourth- higher resting order index
//             let order_id_3 = OrderId {
//                 price_in_ticks: Ticks::new(5),
//                 resting_order_index: RestingOrderIndex::MAX,
//             };
//             manager.check_sorted(side, order_id_3).unwrap();
//             assert_eq!((*manager.last_order_id()).unwrap(), order_id_3);

//             // Fifth- same tick but lower resting order index so fail
//             let order_id_4 = OrderId {
//                 price_in_ticks: Ticks::new(5),
//                 resting_order_index: RestingOrderIndex::new(1),
//             };
//             assert!(manager.check_sorted(side, order_id_4).is_err());
//             assert_eq!((*manager.last_order_id()).unwrap(), order_id_3);
//         }

//         #[test]
//         pub fn test_bids_and_asks_mixed_but_respective_sequence_maintained() {
//             let mut manager = RemoveMultipleManager::new(0, 0);

//             // Insert bid, ask, bid, ask in correct sequence,
//             // then add bid, ask in wrong sequence

//             // 0. Bid
//             let order_id_0 = OrderId {
//                 price_in_ticks: Ticks::new(4),
//                 resting_order_index: RestingOrderIndex::ZERO,
//             };
//             manager.check_sorted(Side::Bid, order_id_0).unwrap();
//             assert_eq!((*manager.last_order_id()).unwrap(), order_id_0);
//             assert_eq!(manager.side, Side::Bid);

//             // 1. Ask
//             let order_id_1 = OrderId {
//                 price_in_ticks: Ticks::new(7),
//                 resting_order_index: RestingOrderIndex::ZERO,
//             };
//             manager.check_sorted(Side::Ask, order_id_1).unwrap();
//             assert_eq!((*manager.last_order_id()).unwrap(), order_id_1);
//             assert_eq!(manager.side, Side::Ask);

//             // 2. Bid
//             let order_id_2 = OrderId {
//                 price_in_ticks: Ticks::new(3),
//                 resting_order_index: RestingOrderIndex::ZERO,
//             };
//             manager.check_sorted(Side::Bid, order_id_2).unwrap();
//             assert_eq!((*manager.last_order_id()).unwrap(), order_id_2);
//             assert_eq!(manager.side, Side::Bid);

//             // 3. Ask
//             let order_id_3 = OrderId {
//                 price_in_ticks: Ticks::new(8),
//                 resting_order_index: RestingOrderIndex::ZERO,
//             };
//             manager.check_sorted(Side::Ask, order_id_3).unwrap();
//             assert_eq!((*manager.last_order_id()).unwrap(), order_id_3);
//             assert_eq!(manager.side, Side::Ask);

//             // 4. Out of order bid
//             let order_id_4 = OrderId {
//                 price_in_ticks: Ticks::new(5),
//                 resting_order_index: RestingOrderIndex::ZERO,
//             };

//             assert!(manager.check_sorted(Side::Bid, order_id_4).is_err());

//             // Since side changed, last_order_id will change
//             assert_eq!(manager.side, Side::Bid);
//             assert_eq!((*manager.last_order_id()).unwrap(), order_id_2);

//             // 5. Out of order ask
//             let order_id_5 = OrderId {
//                 price_in_ticks: Ticks::new(6),
//                 resting_order_index: RestingOrderIndex::ZERO,
//             };
//             // Last order ID did not change but side changed
//             assert!(manager.check_sorted(Side::Ask, order_id_5).is_err());

//             // Since side changed, last_order_id will change
//             assert_eq!(manager.side, Side::Ask);
//             assert_eq!((*manager.last_order_id()).unwrap(), order_id_3);
//         }
//     }
// }
