use crate::state::{
    order::{group_position::GroupPosition, order_id::OrderId},
    remove::{IGroupPositionRemover, IOrderSequentialRemover},
    ArbContext,
};

use super::{IGroupPositionLookupRemover, IOrderLookupRemoverInner, IOuterIndexLookupRemover};

/// Utility to lookup whether a set of order IDs are active and to optionally deactivate them.
/// Successive order ids passed to find() must be move away from the centre, i.e.
/// in descending order for bids and in ascending order for asks.
pub trait IOrderLookupRemover<'a>: IOrderLookupRemoverInner<'a> {
    /// Paginate to the given order id and check whether it is active.
    ///
    ///
    /// # Arguments
    ///
    /// * `ctx`
    /// * `order_id` - Order to search
    ///
    /// # Returns
    ///
    /// * `true` if the order id is present in the book
    /// * `false` if the order id is not present
    fn find(&mut self, ctx: &mut ArbContext, order_id: OrderId) -> bool {
        let price = order_id.price_in_ticks;
        let outer_index = price.outer_index();
        let previous_outer_index = self.outer_index();

        if self.pending_write() {
            // previous_outer_index is guaranteed to exist if pending_write is true
            let previous_outer_index = previous_outer_index.unwrap();
            if previous_outer_index != outer_index {
                self.group_position_remover()
                    .write_to_slot(ctx, previous_outer_index);

                *self.pending_write_mut() = false;
            }
        }
        // Prevous outer index is None or not equal to the new outer index
        if previous_outer_index != Some(outer_index) {
            let outer_index_found = self.outer_index_remover_mut().find(ctx, outer_index);
            if !outer_index_found {
                return false;
            }
            self.group_position_remover_mut()
                .load_outer_index(ctx, outer_index);
        }
        self.group_position_remover_mut()
            .find(GroupPosition::from(&order_id))
    }

    /// Remove the last searched order id from the book
    ///
    /// # Arguments
    ///
    /// * `ctx`
    fn remove(&mut self, ctx: &mut ArbContext) {
        if let Some(order_id) = self.order_id() {
            let price = order_id.price_in_ticks;
            let group_position = GroupPosition::from(&order_id);

            // Use the sequential remover if this is the outermost active tick.
            // The sequential remover will paginate to the next active tick and
            // update the best market price.
            //
            // Closure of best market price has two subcases
            // * Outermost group closed- sequential remover will decrement
            // outer index count
            // * Outermost group not closed
            if price == *self.best_market_price_inner_mut()
                && self
                    .group_position_remover()
                    .is_lowest_active_bit_on_tick(group_position)
            {
                self.sequential_order_remover().next(ctx);
            } else {
                // Closure will not change the best market price.
                // This has 2 cases
                // * Removing any bit on the outermost group except for the outermost
                // active tick
                // * Removal on an inner bitmap group
                //
                // Group remains active in case 1 but it can close in
                // case 2. If bitmap group remains active we need to write the pending
                // group to slot. Otherwise we can simply remove its outer index.
                //
                self.group_position_remover_mut().remove();

                let group_is_active = self.group_position_remover().is_group_active();
                self.set_pending_write(group_is_active);
                if !group_is_active {
                    self.outer_index_remover_mut().remove();
                }
            }
        }
    }

    /// Concludes the removal. Writes the bitmap group if `pending_write` is true,
    /// updates the outer index count and writes any pending outer index list slots.
    ///
    ///
    /// Slot writes- bitmap_group and index list. Market state is updated in memory, where the
    /// best market price and outer index count is updated.
    ///
    /// # Arguments
    ///
    /// * `ctx`
    fn commit(&mut self, ctx: &mut ArbContext) {
        if let Some(outer_index) = self.outer_index() {
            if self.pending_write() {
                self.group_position_remover()
                    .write_to_slot(ctx, outer_index);
            }
            self.outer_index_remover_mut().commit(ctx);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::{
        quantities::Ticks,
        state::{
            bitmap_group::BitmapGroup, remove::OrderLookupRemover, ContextActions, InnerIndex,
            ListKey, ListSlot, OuterIndex, RestingOrderIndex, Side,
        },
    };

    mod sequential_removals {
        use crate::state::remove::IGroupPositionSequentialRemover;

        use super::*;

        #[test]
        fn test_sequentially_remove_outermost_active_asks() {
            // The behavior should match the sequential remover

            let ctx = &mut ArbContext::new();
            let side = Side::Ask;

            let outer_index_0 = OuterIndex::new(1);
            let mut bitmap_group_0 = BitmapGroup::default();
            bitmap_group_0.inner[0] = 0b1000_0000; // Garbage bit
            bitmap_group_0.inner[1] = 0b0000_0101; // Best market price starts here
            bitmap_group_0.inner[31] = 0b1000_0000;
            bitmap_group_0.write_to_slot(ctx, &outer_index_0);

            let outer_index_1 = OuterIndex::new(2);
            let mut bitmap_group_1 = BitmapGroup::default();
            bitmap_group_1.inner[0] = 0b0000_0001;
            bitmap_group_1.write_to_slot(ctx, &outer_index_1);

            let outer_index_2 = OuterIndex::new(5);
            let mut bitmap_group_2 = BitmapGroup::default();
            bitmap_group_2.inner[0] = 0b0000_0001;
            bitmap_group_2.write_to_slot(ctx, &outer_index_2);

            let mut list_slot = ListSlot::default();
            list_slot.set(0, outer_index_2);
            list_slot.set(1, outer_index_1);
            list_slot.set(2, outer_index_0);
            list_slot.write_to_slot(ctx, &ListKey { index: 0, side });

            let mut outer_index_count = 3;
            let mut best_ask_price = Ticks::from_indices(outer_index_0, InnerIndex::new(1));

            let mut remover =
                OrderLookupRemover::new(side, &mut best_ask_price, &mut outer_index_count);

            let order_id_0 = OrderId {
                price_in_ticks: Ticks::from_indices(outer_index_0, InnerIndex::new(1)),
                resting_order_index: RestingOrderIndex::new(0),
            };
            let order_id_1 = OrderId {
                price_in_ticks: Ticks::from_indices(outer_index_0, InnerIndex::new(1)),
                resting_order_index: RestingOrderIndex::new(2),
            };
            let order_id_2 = OrderId {
                price_in_ticks: Ticks::from_indices(outer_index_0, InnerIndex::new(31)),
                resting_order_index: RestingOrderIndex::new(7),
            };
            let order_id_3 = OrderId {
                price_in_ticks: Ticks::from_indices(outer_index_1, InnerIndex::new(0)),
                resting_order_index: RestingOrderIndex::new(0),
            };
            let order_id_4 = OrderId {
                price_in_ticks: Ticks::from_indices(outer_index_2, InnerIndex::new(0)),
                resting_order_index: RestingOrderIndex::new(0),
            };

            // 1- remove first

            assert_eq!(remover.find(ctx, order_id_0), true);
            assert_eq!(remover.order_id().unwrap(), order_id_0);
            assert_eq!(remover.pending_write, false);
            assert_eq!(remover.outer_index().unwrap(), outer_index_0);
            assert_eq!(remover.outer_index_remover.outer_index_count(), 2);

            remover.remove(ctx);
            assert_eq!(remover.order_id().unwrap(), order_id_1); // move to next active order
            assert_eq!(remover.pending_write, true);
            assert_eq!(remover.outer_index().unwrap(), outer_index_0);
            assert_eq!(remover.outer_index_remover.outer_index_count(), 2);

            // still on the same group
            let mut expected_bitmap_group_0 = BitmapGroup::default();
            expected_bitmap_group_0.inner[0] = 0b1000_0000; // Garbage bit
            expected_bitmap_group_0.inner[1] = 0b0000_0100; // i = 0 deactivated
            expected_bitmap_group_0.inner[31] = 0b1000_0000;
            assert_eq!(
                remover
                    .group_position_remover
                    .active_group_position_iterator
                    .bitmap_group,
                expected_bitmap_group_0
            );
            assert_eq!(
                *remover.best_market_price,
                order_id_1.price_in_ticks // no change in market price
            );

            // 2- remove last item from best market price

            assert_eq!(remover.find(ctx, order_id_1), true);
            assert_eq!(remover.order_id().unwrap(), order_id_1);
            assert_eq!(remover.pending_write, true); // because we didn't write after previous remove
            assert_eq!(remover.outer_index().unwrap(), outer_index_0);
            assert_eq!(remover.outer_index_remover.outer_index_count(), 2);

            remover.remove(ctx);
            assert_eq!(remover.order_id().unwrap(), order_id_2); // moved to the next active order id

            assert_eq!(remover.pending_write, false); // false because best market price updated
            expected_bitmap_group_0.inner[1] = 0b0000_0000;
            expected_bitmap_group_0.inner[31] = 0b1000_0000;
            assert_eq!(
                remover
                    .group_position_remover
                    .active_group_position_iterator
                    .bitmap_group,
                expected_bitmap_group_0
            );
            assert_eq!(
                *remover.best_market_price,
                order_id_2.price_in_ticks // changed
            );
            assert_eq!(remover.outer_index().unwrap(), outer_index_0);
            assert_eq!(remover.outer_index_remover.outer_index_count(), 2);

            // 3- find and remove from same group with different inner index
            assert_eq!(remover.find(ctx, order_id_2), true);
            assert_eq!(remover.pending_write, false);
            assert_eq!(remover.outer_index().unwrap(), outer_index_0);
            assert_eq!(remover.outer_index_remover.outer_index_count(), 2);

            remover.remove(ctx);
            assert_eq!(remover.order_id().unwrap(), order_id_3);
            assert_eq!(remover.pending_write, false);

            let mut expected_bitmap_group_1 = BitmapGroup::default();
            expected_bitmap_group_1.inner[0] = 0b0000_0001;
            assert_eq!(
                remover
                    .group_position_remover
                    .active_group_position_iterator
                    .bitmap_group,
                expected_bitmap_group_1
            );
            assert_eq!(
                *remover.best_market_price,
                order_id_3.price_in_ticks // changed
            );
            assert_eq!(remover.outer_index().unwrap(), outer_index_1);
            assert_eq!(remover.outer_index_remover.outer_index_count(), 1);
            assert_eq!(remover.outer_index_remover.cached_outer_indices, vec![]);

            // 4- find and remove from next group
            assert_eq!(remover.find(ctx, order_id_3), true);
            assert_eq!(remover.pending_write, false);
            assert_eq!(remover.outer_index().unwrap(), outer_index_1);
            assert_eq!(remover.outer_index_remover.outer_index_count(), 1);
            assert_eq!(remover.outer_index_remover.cached_outer_indices, vec![]);

            remover.remove(ctx);
            assert_eq!(remover.order_id().unwrap(), order_id_4);
            assert_eq!(remover.pending_write, false);

            let mut expected_bitmap_group_2 = BitmapGroup::default();
            expected_bitmap_group_2.inner[0] = 0b0000_0001;
            assert_eq!(
                remover
                    .group_position_remover
                    .active_group_position_iterator
                    .bitmap_group,
                expected_bitmap_group_2
            );
            assert_eq!(
                *remover.best_market_price,
                order_id_4.price_in_ticks // changed
            );
            assert_eq!(remover.outer_index().unwrap(), outer_index_2);
            assert_eq!(remover.outer_index_remover.outer_index_count(), 0);
            assert_eq!(remover.outer_index_remover.cached_outer_indices, vec![]);

            // 5- find and remove last active order
            assert_eq!(remover.find(ctx, order_id_4), true);
            assert_eq!(remover.pending_write, false);
            assert_eq!(remover.outer_index().unwrap(), outer_index_2);
            assert_eq!(remover.outer_index_remover.outer_index_count(), 0);
            assert_eq!(remover.outer_index_remover.cached_outer_indices, vec![]);

            remover.remove(ctx);

            // updated spec- order id will be None when all active bits have been removed
            // but group position will hold the last element of the group
            assert_eq!(remover.order_id(), None);
            assert_eq!(
                remover.group_position().unwrap(),
                GroupPosition {
                    inner_index: InnerIndex::MAX,
                    resting_order_index: RestingOrderIndex::MAX
                }
            );
            assert_eq!(remover.pending_write, false);

            expected_bitmap_group_2 = BitmapGroup::default();
            expected_bitmap_group_2.inner[0] = 0b0000_0000;
            assert_eq!(
                remover
                    .group_position_remover
                    .active_group_position_iterator
                    .bitmap_group,
                expected_bitmap_group_2
            );
            assert_eq!(
                *remover.best_market_price,
                order_id_4.price_in_ticks // no change because last tick was exhausted
            );
            assert_eq!(remover.outer_index(), None);
            assert_eq!(remover.outer_index_remover.outer_index_count(), 0);
            assert_eq!(remover.outer_index_remover.cached_outer_indices, vec![]);
        }

        #[test]
        fn test_sequentially_remove_outermost_active_bids() {
            // The behavior should match the sequential remover

            let ctx = &mut ArbContext::new();
            let side = Side::Bid;

            let outer_index_0 = OuterIndex::new(5);
            let mut bitmap_group_0 = BitmapGroup::default();
            // This holds the last bit at 255
            bitmap_group_0.inner[0] = 0b1000_0000;
            bitmap_group_0.inner[1] = 0b0000_0101; // Best market price starts here
            bitmap_group_0.inner[31] = 0b0000_0001; // Garbage bit
            bitmap_group_0.write_to_slot(ctx, &outer_index_0);

            let outer_index_1 = OuterIndex::new(2);
            let mut bitmap_group_1 = BitmapGroup::default();
            bitmap_group_1.inner[0] = 0b0000_0001;
            bitmap_group_1.write_to_slot(ctx, &outer_index_1);

            let outer_index_2 = OuterIndex::new(1);
            let mut bitmap_group_2 = BitmapGroup::default();
            bitmap_group_2.inner[0] = 0b0000_0001;
            bitmap_group_2.write_to_slot(ctx, &outer_index_2);

            let mut list_slot = ListSlot::default();
            list_slot.set(0, outer_index_2);
            list_slot.set(1, outer_index_1);
            list_slot.set(2, outer_index_0);
            list_slot.write_to_slot(ctx, &ListKey { index: 0, side });

            let mut outer_index_count = 3;
            let mut best_market_price = Ticks::from_indices(outer_index_0, InnerIndex::new(1));

            let mut remover =
                OrderLookupRemover::new(side, &mut best_market_price, &mut outer_index_count);

            let order_id_0 = OrderId {
                price_in_ticks: Ticks::from_indices(outer_index_0, InnerIndex::new(1)),
                resting_order_index: RestingOrderIndex::new(0),
            };
            let order_id_1 = OrderId {
                price_in_ticks: Ticks::from_indices(outer_index_0, InnerIndex::new(1)),
                resting_order_index: RestingOrderIndex::new(2),
            };
            let order_id_2 = OrderId {
                price_in_ticks: Ticks::from_indices(outer_index_0, InnerIndex::new(0)),
                resting_order_index: RestingOrderIndex::new(7),
            };
            let order_id_3 = OrderId {
                price_in_ticks: Ticks::from_indices(outer_index_1, InnerIndex::new(0)),
                resting_order_index: RestingOrderIndex::new(0),
            };
            let order_id_4 = OrderId {
                price_in_ticks: Ticks::from_indices(outer_index_2, InnerIndex::new(0)),
                resting_order_index: RestingOrderIndex::new(0),
            };

            // 1- remove first

            assert_eq!(remover.find(ctx, order_id_0), true);
            assert_eq!(remover.order_id().unwrap(), order_id_0);
            assert_eq!(remover.pending_write, false);
            assert_eq!(remover.outer_index().unwrap(), outer_index_0);
            assert_eq!(remover.outer_index_remover.outer_index_count(), 2);

            remover.remove(ctx);
            assert_eq!(remover.order_id().unwrap(), order_id_1); // move to next active order
            assert_eq!(remover.pending_write, true);
            assert_eq!(remover.outer_index().unwrap(), outer_index_0);
            assert_eq!(remover.outer_index_remover.outer_index_count(), 2);

            // still on the same group
            let mut expected_bitmap_group_0 = BitmapGroup::default();
            expected_bitmap_group_0.inner[0] = 0b1000_0000;
            expected_bitmap_group_0.inner[1] = 0b0000_0100; // i = 0 deactivated
            expected_bitmap_group_0.inner[31] = 0b0000_0001; // Garbage bit
            assert_eq!(
                remover
                    .group_position_remover
                    .active_group_position_iterator
                    .bitmap_group,
                expected_bitmap_group_0
            );
            assert_eq!(
                *remover.best_market_price,
                order_id_1.price_in_ticks // no change in market price
            );

            // 2- remove last item from best market price

            assert_eq!(remover.find(ctx, order_id_1), true);
            assert_eq!(remover.order_id().unwrap(), order_id_1);
            assert_eq!(remover.pending_write, true); // because we didn't write after previous remove
            assert_eq!(remover.outer_index().unwrap(), outer_index_0);
            assert_eq!(remover.outer_index_remover.outer_index_count(), 2);

            remover.remove(ctx);

            // group position is undefined but last group position is correct
            // need to set group position

            assert_eq!(remover.order_id().unwrap(), order_id_2); // moved to the next active order id

            assert_eq!(remover.pending_write, false); // false because best market price updated
            expected_bitmap_group_0.inner[0] = 0b1000_0000;
            expected_bitmap_group_0.inner[1] = 0b0000_0000;
            expected_bitmap_group_0.inner[31] = 0b0000_0001;
            assert_eq!(
                remover
                    .group_position_remover
                    .active_group_position_iterator
                    .bitmap_group,
                expected_bitmap_group_0
            );
            assert_eq!(
                *remover.best_market_price,
                order_id_2.price_in_ticks // changed
            );
            assert_eq!(remover.outer_index().unwrap(), outer_index_0);
            assert_eq!(remover.outer_index_remover.outer_index_count(), 2);

            // 3- find and remove from same group with different inner index
            assert_eq!(remover.find(ctx, order_id_2), true);
            assert_eq!(remover.pending_write, false);
            assert_eq!(remover.outer_index().unwrap(), outer_index_0);
            assert_eq!(remover.outer_index_remover.outer_index_count(), 2);

            remover.remove(ctx);
            assert_eq!(remover.order_id().unwrap(), order_id_3);
            assert_eq!(remover.pending_write, false);

            let mut expected_bitmap_group_1 = BitmapGroup::default();
            expected_bitmap_group_1.inner[0] = 0b0000_0001;
            assert_eq!(
                remover
                    .group_position_remover
                    .active_group_position_iterator
                    .bitmap_group,
                expected_bitmap_group_1
            );
            assert_eq!(
                *remover.best_market_price,
                order_id_3.price_in_ticks // changed
            );
            assert_eq!(remover.outer_index().unwrap(), outer_index_1);
            assert_eq!(remover.outer_index_remover.outer_index_count(), 1);
            assert_eq!(remover.outer_index_remover.cached_outer_indices, vec![]);

            // 4- find and remove from next group
            assert_eq!(remover.find(ctx, order_id_3), true);
            assert_eq!(remover.pending_write, false);
            assert_eq!(remover.outer_index().unwrap(), outer_index_1);
            assert_eq!(remover.outer_index_remover.outer_index_count(), 1);
            assert_eq!(remover.outer_index_remover.cached_outer_indices, vec![]);

            remover.remove(ctx);
            assert_eq!(remover.order_id().unwrap(), order_id_4);
            assert_eq!(remover.pending_write, false);

            let mut expected_bitmap_group_2 = BitmapGroup::default();
            expected_bitmap_group_2.inner[0] = 0b0000_0001;
            assert_eq!(
                remover
                    .group_position_remover
                    .active_group_position_iterator
                    .bitmap_group,
                expected_bitmap_group_2
            );
            assert_eq!(
                *remover.best_market_price,
                order_id_4.price_in_ticks // changed
            );
            assert_eq!(remover.outer_index().unwrap(), outer_index_2);
            assert_eq!(remover.outer_index_remover.outer_index_count(), 0);
            assert_eq!(remover.outer_index_remover.cached_outer_indices, vec![]);

            // 5- find and remove last active order
            assert_eq!(remover.find(ctx, order_id_4), true);
            assert_eq!(remover.pending_write, false);
            assert_eq!(remover.outer_index().unwrap(), outer_index_2);
            assert_eq!(remover.outer_index_remover.outer_index_count(), 0);
            assert_eq!(remover.outer_index_remover.cached_outer_indices, vec![]);

            remover.remove(ctx);
            assert_eq!(remover.order_id(), None);
            assert_eq!(
                remover.group_position().unwrap(),
                GroupPosition {
                    inner_index: InnerIndex::ZERO,
                    resting_order_index: RestingOrderIndex::MAX
                }
            );
            assert_eq!(remover.pending_write, false);

            expected_bitmap_group_2 = BitmapGroup::default();
            expected_bitmap_group_2.inner[0] = 0b0000_0000;
            assert_eq!(
                remover
                    .group_position_remover
                    .active_group_position_iterator
                    .bitmap_group,
                expected_bitmap_group_2
            );
            assert_eq!(
                *remover.best_market_price,
                order_id_4.price_in_ticks // no change because last tick was exhausted
            );
            assert_eq!(remover.outer_index(), None);
            assert_eq!(remover.outer_index_remover.outer_index_count(), 0);
            assert_eq!(remover.outer_index_remover.cached_outer_indices, vec![]);
        }

        #[test]
        fn test_no_overflow_in_sequential_removes() {
            let ctx = &mut ArbContext::new();
            let side = Side::Ask;

            let outer_index_0 = OuterIndex::new(1);
            let mut bitmap_group_0 = BitmapGroup::default();
            bitmap_group_0.inner[31] = 0b1000_0000;
            bitmap_group_0.write_to_slot(ctx, &outer_index_0);

            let outer_index_1 = OuterIndex::new(2);
            let mut bitmap_group_1 = BitmapGroup::default();
            bitmap_group_1.inner[0] = 0b0000_0001;
            bitmap_group_1.write_to_slot(ctx, &outer_index_1);

            let mut list_slot = ListSlot::default();
            list_slot.set(0, outer_index_1);
            list_slot.set(1, outer_index_0);
            list_slot.write_to_slot(ctx, &ListKey { index: 0, side });

            let mut outer_index_count = 2;

            let order_id_0 = OrderId {
                price_in_ticks: Ticks::from_indices(outer_index_0, InnerIndex::new(31)),
                resting_order_index: RestingOrderIndex::new(7),
            };
            let order_id_1 = OrderId {
                price_in_ticks: Ticks::from_indices(outer_index_1, InnerIndex::new(0)),
                resting_order_index: RestingOrderIndex::new(0),
            };
            let mut best_ask_price = order_id_0.price_in_ticks;

            let mut remover =
                OrderLookupRemover::new(side, &mut best_ask_price, &mut outer_index_count);

            assert_eq!(remover.find(ctx, order_id_0), true);
            remover.remove(ctx);

            println!("group position {:?}", remover.group_position());

            println!(
                "last group position {:?}",
                remover
                    .group_position_remover
                    .previous_position_to_deactivate()
            );

            assert_eq!(remover.order_id().unwrap(), order_id_1); // move to next active order
            assert_eq!(*remover.best_market_price, order_id_1.price_in_ticks);
        }

        #[test]
        fn test_no_underflow_in_sequential_removes() {
            // index = 0 is reached when next() by wrapping addition on index = 255
            // when we reach the end of the group without finding any active bit.
            let ctx = &mut ArbContext::new();
            let side = Side::Ask;

            let outer_index_0 = OuterIndex::new(1);
            let mut bitmap_group_0 = BitmapGroup::default();
            bitmap_group_0.inner[31] = 0b0100_0000;
            bitmap_group_0.write_to_slot(ctx, &outer_index_0);

            let mut list_slot = ListSlot::default();
            list_slot.set(0, outer_index_0);
            list_slot.write_to_slot(ctx, &ListKey { index: 0, side });

            let mut outer_index_count = 1;

            let order_id_0 = OrderId {
                price_in_ticks: Ticks::from_indices(outer_index_0, InnerIndex::new(31)),
                resting_order_index: RestingOrderIndex::new(6),
            };
            let mut best_ask_price = order_id_0.price_in_ticks;

            let mut remover =
                OrderLookupRemover::new(side, &mut best_ask_price, &mut outer_index_count);

            assert_eq!(remover.find(ctx, order_id_0), true);
            remover.remove(ctx);

            assert_eq!(remover.order_id(), None); // move to next active order
            assert_eq!(*remover.best_market_price, order_id_0.price_in_ticks);
        }
    }

    mod random_removals {
        #[test]
        fn test_lookup_and_remove_across_groups_for_asks() {}
    }
}
