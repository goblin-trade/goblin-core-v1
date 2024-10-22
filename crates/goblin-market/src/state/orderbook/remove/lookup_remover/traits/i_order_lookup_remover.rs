use crate::state::{
    order::{group_position::GroupPosition, order_id::OrderId},
    remove::{IGroupPositionRemover, IOrderSequentialRemover},
    ArbContext, Side,
};

use super::{IGroupPositionLookupRemover, IOrderLookupRemoverInner, IOuterIndexLookupRemover};

/// Utility to lookup whether a set of order IDs are active and to optionally deactivate them.
/// Successive order ids passed to find() must be move away from the centre, i.e.
/// in descending order for bids and in ascending order for asks.
pub trait IOrderLookupRemover<'a>: IOrderLookupRemoverInner<'a> {
    /// Paginate to the given order id and check whether it is active.
    ///
    /// # Externally ensure
    ///
    /// * Order ids are sorted to move away from the centre
    /// * Do not pass an order id whose outer index is greater than outer
    /// index of best market price
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

            // If next order id is on the same group, don't write group to slot yet
            if previous_outer_index != outer_index {
                self.group_position_remover()
                    .write_to_slot(ctx, previous_outer_index);

                *self.pending_write_mut() = false;
            }
        }

        let outer_index_found = self.outer_index_remover_mut().find_v2(ctx, outer_index);
        if !outer_index_found {
            return false;
        }
        // Load group if previously loaded outer index is None or not equal to the current one
        if Some(outer_index) != previous_outer_index {
            self.group_position_remover_mut()
                .load_outer_index(ctx, outer_index);
        }

        // // Prevous outer index is None or not equal to the new outer index
        // if previous_outer_index != Some(outer_index) {
        //     // Above condition ensures that outer_index_remover_mut().find() is not
        //     // called again for the same outer index
        //     let outer_index_found = self.outer_index_remover_mut().find_next(ctx, outer_index);
        //     if !outer_index_found {
        //         return false;
        //     }
        //     self.group_position_remover_mut()
        //         .load_outer_index(ctx, outer_index);
        // }

        self.group_position_remover_mut()
            .find(GroupPosition::from(&order_id))
    }

    /// Remove the last searched order id from the book
    ///
    /// # Arguments
    ///
    /// * `ctx`
    fn remove(&mut self, ctx: &mut ArbContext) {
        if let Some(order_id) = self.order_id_to_remove() {
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
            bitmap_group::BitmapGroup, order::order_id, remove::OrderLookupRemover, ContextActions,
            InnerIndex, ListKey, ListSlot, OuterIndex, RestingOrderIndex, Side,
        },
    };

    fn write_outer_indices(ctx: &mut ArbContext, side: Side, outer_indices: Vec<OuterIndex>) {
        let slot_count = outer_indices.len() / 16;

        for slot_index in 0..=slot_count {
            let mut list_slot = ListSlot::default();
            let slot_key = ListKey {
                side,
                index: slot_index as u16,
            };

            let end_outer_index_position = outer_indices.len() - slot_index * 16;

            for outer_index_position in 0..end_outer_index_position {
                let inner_slot_index = 16 * slot_index + outer_index_position;
                let outer_index = outer_indices.get(inner_slot_index).unwrap();
                list_slot.set(outer_index_position, *outer_index);
            }
            println!("writing list slot {:?}", list_slot.inner);
            list_slot.write_to_slot(ctx, &slot_key);
        }
    }

    fn write_order_ids(ctx: &mut ArbContext, order_ids: Vec<OrderId>) {
        for order_id in order_ids {
            let outer_index = order_id.price_in_ticks.outer_index();

            let mut bitmap_group = BitmapGroup::new_from_slot(ctx, outer_index);
            bitmap_group.activate(GroupPosition::from(&order_id));
            bitmap_group.write_to_slot(ctx, &outer_index);
        }
    }

    mod sequential_removals {
        use super::*;

        #[test]
        fn test_sequentially_remove_outermost_active_asks() {
            let ctx = &mut ArbContext::new();
            let side = Side::Ask;

            let mut outer_index_count = 3;
            let outer_index_0 = OuterIndex::new(1);
            let outer_index_1 = OuterIndex::new(2);
            let outer_index_2 = OuterIndex::new(5);
            write_outer_indices(ctx, side, vec![outer_index_2, outer_index_1, outer_index_0]);

            let mut bitmap_group_0 = BitmapGroup::default();
            bitmap_group_0.inner[0] = 0b1000_0000; // Garbage bit
            bitmap_group_0.inner[1] = 0b0000_0101; // Best market price starts here
            bitmap_group_0.inner[31] = 0b1000_0000;
            bitmap_group_0.write_to_slot(ctx, &outer_index_0);

            let mut bitmap_group_1 = BitmapGroup::default();
            bitmap_group_1.inner[0] = 0b0000_0001;
            bitmap_group_1.write_to_slot(ctx, &outer_index_1);

            let mut bitmap_group_2 = BitmapGroup::default();
            bitmap_group_2.inner[0] = 0b0000_0001;
            bitmap_group_2.write_to_slot(ctx, &outer_index_2);

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

            let mut best_market_price = order_id_0.price_in_ticks;
            let mut remover =
                OrderLookupRemover::new(side, &mut best_market_price, &mut outer_index_count);

            // 1- remove first
            assert_eq!(remover.find(ctx, order_id_0), true);
            assert_eq!(remover.pending_write, false);
            assert_eq!(remover.order_id_to_remove().unwrap(), order_id_0);
            assert_eq!(remover.outer_index().unwrap(), outer_index_0);
            assert_eq!(remover.outer_index_remover.unread_outer_indices(), 2);

            remover.remove(ctx);
            assert_eq!(remover.pending_write, true);
            assert_eq!(remover.order_id_to_remove().unwrap(), order_id_1); // move to next active order
            assert_eq!(remover.outer_index().unwrap(), outer_index_0);
            assert_eq!(remover.outer_index_remover.unread_outer_indices(), 2);

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
            assert_eq!(remover.pending_write, true); // because we didn't write after previous remove
            assert_eq!(remover.order_id_to_remove().unwrap(), order_id_1);
            assert_eq!(remover.outer_index().unwrap(), outer_index_0);
            assert_eq!(remover.outer_index_remover.unread_outer_indices(), 2);

            remover.remove(ctx);
            assert_eq!(remover.pending_write, false); // false because best market price updated
            assert_eq!(remover.order_id_to_remove().unwrap(), order_id_2); // moved to the next active order id

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
            assert_eq!(remover.outer_index_remover.unread_outer_indices(), 2);

            // 3- find and remove from same group with different inner index
            assert_eq!(remover.find(ctx, order_id_2), true);
            assert_eq!(remover.pending_write, false);
            assert_eq!(remover.outer_index().unwrap(), outer_index_0);
            assert_eq!(remover.outer_index_remover.unread_outer_indices(), 2);

            remover.remove(ctx);
            assert_eq!(remover.order_id_to_remove().unwrap(), order_id_3);
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
            assert_eq!(remover.outer_index_remover.unread_outer_indices(), 1);
            assert_eq!(remover.outer_index_remover.cached_outer_indices, vec![]);

            // 4- find and remove from next group
            assert_eq!(remover.find(ctx, order_id_3), true);
            assert_eq!(remover.pending_write, false);
            assert_eq!(remover.outer_index().unwrap(), outer_index_1);
            assert_eq!(remover.outer_index_remover.unread_outer_indices(), 1);
            assert_eq!(remover.outer_index_remover.cached_outer_indices, vec![]);

            remover.remove(ctx);
            assert_eq!(remover.order_id_to_remove().unwrap(), order_id_4);
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
            assert_eq!(remover.outer_index_remover.unread_outer_indices(), 0);
            assert_eq!(remover.outer_index_remover.cached_outer_indices, vec![]);

            // 5- find and remove last active order
            assert_eq!(remover.find(ctx, order_id_4), true);
            assert_eq!(remover.pending_write, false);
            assert_eq!(remover.outer_index_remover.unread_outer_indices(), 0);
            assert_eq!(remover.outer_index().unwrap(), outer_index_2);
            assert_eq!(remover.outer_index_remover.cached_outer_indices, vec![]);

            remover.remove(ctx);
            assert_eq!(remover.order_id_to_remove(), None);
            assert_eq!(remover.outer_index(), None);
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
            assert_eq!(remover.outer_index_remover.unread_outer_indices(), 0);
            assert_eq!(remover.outer_index_remover.cached_outer_indices, vec![]);
        }

        #[test]
        fn test_sequentially_remove_outermost_active_bids() {
            let ctx = &mut ArbContext::new();
            let side = Side::Bid;

            let mut outer_index_count = 3;
            let outer_index_0 = OuterIndex::new(5);
            let outer_index_1 = OuterIndex::new(2);
            let outer_index_2 = OuterIndex::new(1);
            write_outer_indices(ctx, side, vec![outer_index_2, outer_index_1, outer_index_0]);

            let mut bitmap_group_0 = BitmapGroup::default();
            // This holds the last bit at 255
            bitmap_group_0.inner[0] = 0b1000_0000;
            bitmap_group_0.inner[1] = 0b0000_0101; // Best market price starts here
            bitmap_group_0.inner[31] = 0b0000_0001; // Garbage bit
            bitmap_group_0.write_to_slot(ctx, &outer_index_0);

            let mut bitmap_group_1 = BitmapGroup::default();
            bitmap_group_1.inner[0] = 0b0000_0001;
            bitmap_group_1.write_to_slot(ctx, &outer_index_1);

            let mut bitmap_group_2 = BitmapGroup::default();
            bitmap_group_2.inner[0] = 0b0000_0001;
            bitmap_group_2.write_to_slot(ctx, &outer_index_2);

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

            let mut best_market_price = order_id_0.price_in_ticks;

            let mut remover =
                OrderLookupRemover::new(side, &mut best_market_price, &mut outer_index_count);

            // 1- remove first

            assert_eq!(remover.find(ctx, order_id_0), true);
            assert_eq!(remover.pending_write, false);
            assert_eq!(remover.order_id_to_remove().unwrap(), order_id_0);
            assert_eq!(remover.outer_index().unwrap(), outer_index_0);
            assert_eq!(remover.outer_index_remover.unread_outer_indices(), 2);

            remover.remove(ctx);
            assert_eq!(remover.pending_write, true);
            assert_eq!(remover.order_id_to_remove().unwrap(), order_id_1); // move to next active order
            assert_eq!(remover.outer_index().unwrap(), outer_index_0);
            assert_eq!(remover.outer_index_remover.unread_outer_indices(), 2);

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
            assert_eq!(remover.pending_write, true); // because we didn't write after previous remove
            assert_eq!(remover.order_id_to_remove().unwrap(), order_id_1);
            assert_eq!(remover.outer_index().unwrap(), outer_index_0);
            assert_eq!(remover.outer_index_remover.unread_outer_indices(), 2);

            remover.remove(ctx);

            // group position is undefined but last group position is correct
            // need to set group position

            assert_eq!(remover.order_id_to_remove().unwrap(), order_id_2); // moved to the next active order id

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
            assert_eq!(remover.outer_index_remover.unread_outer_indices(), 2);

            // 3- find and remove from same group with different inner index
            assert_eq!(remover.find(ctx, order_id_2), true);
            assert_eq!(remover.pending_write, false);
            assert_eq!(remover.outer_index().unwrap(), outer_index_0);
            assert_eq!(remover.outer_index_remover.unread_outer_indices(), 2);

            remover.remove(ctx);
            assert_eq!(remover.order_id_to_remove().unwrap(), order_id_3);
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
            assert_eq!(remover.outer_index_remover.unread_outer_indices(), 1);
            assert_eq!(remover.outer_index_remover.cached_outer_indices, vec![]);

            // 4- find and remove from next group
            assert_eq!(remover.find(ctx, order_id_3), true);
            assert_eq!(remover.pending_write, false);
            assert_eq!(remover.outer_index().unwrap(), outer_index_1);
            assert_eq!(remover.outer_index_remover.unread_outer_indices(), 1);
            assert_eq!(remover.outer_index_remover.cached_outer_indices, vec![]);

            remover.remove(ctx);
            assert_eq!(remover.order_id_to_remove().unwrap(), order_id_4);
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
            assert_eq!(remover.outer_index_remover.unread_outer_indices(), 0);
            assert_eq!(remover.outer_index_remover.cached_outer_indices, vec![]);

            // 5- find and remove last active order
            assert_eq!(remover.find(ctx, order_id_4), true);
            assert_eq!(remover.pending_write, false);
            assert_eq!(remover.outer_index().unwrap(), outer_index_2);
            assert_eq!(remover.outer_index_remover.unread_outer_indices(), 0);
            assert_eq!(remover.outer_index_remover.cached_outer_indices, vec![]);

            remover.remove(ctx);
            assert_eq!(remover.order_id_to_remove(), None);
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
            assert_eq!(remover.outer_index_remover.unread_outer_indices(), 0);
            assert_eq!(remover.outer_index_remover.cached_outer_indices, vec![]);
        }
    }

    mod random_removals {
        use super::*;

        #[test]
        fn test_lookup_asks_sequentially_then_randomly() {
            let ctx = &mut ArbContext::new();
            let side = Side::Ask;

            let mut outer_index_count = 2;
            let outer_index_0 = OuterIndex::new(1);
            let outer_index_1 = OuterIndex::new(2);
            write_outer_indices(ctx, side, vec![outer_index_1, outer_index_0]);

            let mut bitmap_group_0 = BitmapGroup::default();
            bitmap_group_0.inner[0] = 0b1000_0000; // Garbage bit
            bitmap_group_0.inner[1] = 0b0000_0101; // Best market price starts here
            bitmap_group_0.inner[31] = 0b1000_0000;
            bitmap_group_0.write_to_slot(ctx, &outer_index_0);

            let mut bitmap_group_1 = BitmapGroup::default();
            bitmap_group_1.inner[0] = 0b0000_0001;
            bitmap_group_1.inner[1] = 0b0000_0001;
            bitmap_group_1.write_to_slot(ctx, &outer_index_1);

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
                price_in_ticks: Ticks::from_indices(outer_index_1, InnerIndex::new(1)),
                resting_order_index: RestingOrderIndex::new(0),
            };

            let mut best_market_price = order_id_0.price_in_ticks;
            let mut remover =
                OrderLookupRemover::new(side, &mut best_market_price, &mut outer_index_count);

            // Sequentially remove order id 0 and 1
            remover.find(ctx, order_id_0);
            remover.remove(ctx);
            remover.find(ctx, order_id_1);
            remover.remove(ctx);

            // Jump to order id 3 and remove it
            remover.find(ctx, order_id_3);
            assert_eq!(remover.order_id_to_remove().unwrap(), order_id_3);
            assert_eq!(remover.outer_index().unwrap(), outer_index_1);
            assert_eq!(
                remover.outer_index_remover.cached_outer_indices,
                vec![outer_index_0]
            );

            let mut expected_bitmap_group_1 = BitmapGroup::default();
            expected_bitmap_group_1.inner[0] = 0b0000_0001;
            expected_bitmap_group_1.inner[1] = 0b0000_0001;
            assert_eq!(
                remover
                    .group_position_remover
                    .active_group_position_iterator
                    .bitmap_group,
                expected_bitmap_group_1
            );

            remover.remove(ctx);
            assert_eq!(remover.order_id_to_remove().unwrap(), order_id_3);
            assert_eq!(remover.outer_index().unwrap(), outer_index_1);
            assert_eq!(
                remover.outer_index_remover.cached_outer_indices,
                vec![outer_index_0]
            );
            // Best market price remains on order id 2
            assert_eq!(*remover.best_market_price, order_id_2.price_in_ticks);

            let mut expected_bitmap_group_1 = BitmapGroup::default();
            expected_bitmap_group_1.inner[0] = 0b0000_0000;
            expected_bitmap_group_1.inner[1] = 0b0000_0001;
            assert_eq!(
                remover
                    .group_position_remover
                    .active_group_position_iterator
                    .bitmap_group,
                expected_bitmap_group_1
            );

            // Remove last item
            remover.find(ctx, order_id_4);
            assert_eq!(remover.order_id_to_remove().unwrap(), order_id_4);
            assert_eq!(remover.outer_index().unwrap(), outer_index_1);
            assert_eq!(
                remover.outer_index_remover.cached_outer_indices,
                vec![outer_index_0]
            );
            let mut expected_bitmap_group_1 = BitmapGroup::default();
            expected_bitmap_group_1.inner[0] = 0b0000_0000;
            expected_bitmap_group_1.inner[1] = 0b0000_0001;
            assert_eq!(
                remover
                    .group_position_remover
                    .active_group_position_iterator
                    .bitmap_group,
                expected_bitmap_group_1
            );

            remover.remove(ctx);

            // Outer index removed because group closed
            assert_eq!(remover.order_id_to_remove(), None);
            assert_eq!(remover.outer_index(), None);
            assert_eq!(
                remover.outer_index_remover.cached_outer_indices,
                vec![outer_index_0]
            );
            // Group position remains unchanged
            assert_eq!(
                remover.group_position().unwrap(),
                GroupPosition::from(&order_id_4)
            );

            // Best market price remains on order id 2
            assert_eq!(*remover.best_market_price, order_id_2.price_in_ticks);

            let mut expected_bitmap_group_1 = BitmapGroup::default();
            expected_bitmap_group_1.inner[0] = 0b0000_0000;
            expected_bitmap_group_1.inner[1] = 0b0000_0000;
            assert_eq!(
                remover
                    .group_position_remover
                    .active_group_position_iterator
                    .bitmap_group,
                expected_bitmap_group_1
            );
        }

        #[test]
        fn test_lookup_bids_sequentially_then_randomly() {
            let ctx = &mut ArbContext::new();
            let side = Side::Bid;

            let mut outer_index_count = 2;
            let outer_index_0 = OuterIndex::new(1);
            let outer_index_1 = OuterIndex::new(2);
            write_outer_indices(ctx, side, vec![outer_index_1, outer_index_0]);

            let mut bitmap_group_0 = BitmapGroup::default();
            bitmap_group_0.inner[0] = 0b1000_0000;
            bitmap_group_0.inner[1] = 0b0000_0101; // Best market price starts here
            bitmap_group_0.inner[31] = 0b1000_0000; // Garbage bit
            bitmap_group_0.write_to_slot(ctx, &outer_index_0);

            let mut bitmap_group_1 = BitmapGroup::default();
            bitmap_group_1.inner[0] = 0b0000_0001;
            bitmap_group_1.inner[1] = 0b0000_0001;
            bitmap_group_1.write_to_slot(ctx, &outer_index_1);

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
                price_in_ticks: Ticks::from_indices(outer_index_1, InnerIndex::new(1)),
                resting_order_index: RestingOrderIndex::new(0),
            };
            let order_id_4 = OrderId {
                price_in_ticks: Ticks::from_indices(outer_index_1, InnerIndex::new(0)),
                resting_order_index: RestingOrderIndex::new(0),
            };

            let mut best_market_price = order_id_0.price_in_ticks;
            let mut remover =
                OrderLookupRemover::new(side, &mut best_market_price, &mut outer_index_count);

            // Sequentially remove order id 0 and 1
            remover.find(ctx, order_id_0);
            remover.remove(ctx);
            remover.find(ctx, order_id_1);
            remover.remove(ctx);

            // Jump to order id 3 and remove it
            remover.find(ctx, order_id_3);
            assert_eq!(remover.order_id_to_remove().unwrap(), order_id_3);
            assert_eq!(remover.outer_index().unwrap(), outer_index_1);
            assert_eq!(
                remover.outer_index_remover.cached_outer_indices,
                vec![outer_index_0]
            );

            let mut expected_bitmap_group_1 = BitmapGroup::default();
            expected_bitmap_group_1.inner[0] = 0b0000_0001;
            expected_bitmap_group_1.inner[1] = 0b0000_0001;
            assert_eq!(
                remover
                    .group_position_remover
                    .active_group_position_iterator
                    .bitmap_group,
                expected_bitmap_group_1
            );

            remover.remove(ctx);
            assert_eq!(remover.order_id_to_remove().unwrap(), order_id_3);
            assert_eq!(remover.outer_index().unwrap(), outer_index_1);
            assert_eq!(
                remover.outer_index_remover.cached_outer_indices,
                vec![outer_index_0]
            );
            // Best market price remains on order id 2
            assert_eq!(*remover.best_market_price, order_id_2.price_in_ticks);

            let mut expected_bitmap_group_1 = BitmapGroup::default();
            expected_bitmap_group_1.inner[0] = 0b0000_0001;
            expected_bitmap_group_1.inner[1] = 0b0000_0000;
            assert_eq!(
                remover
                    .group_position_remover
                    .active_group_position_iterator
                    .bitmap_group,
                expected_bitmap_group_1
            );

            // Remove last item
            remover.find(ctx, order_id_4);
            assert_eq!(remover.order_id_to_remove().unwrap(), order_id_4);
            assert_eq!(remover.outer_index().unwrap(), outer_index_1);
            assert_eq!(
                remover.outer_index_remover.cached_outer_indices,
                vec![outer_index_0]
            );
            let mut expected_bitmap_group_1 = BitmapGroup::default();
            expected_bitmap_group_1.inner[0] = 0b0000_0001;
            expected_bitmap_group_1.inner[1] = 0b0000_0000;
            assert_eq!(
                remover
                    .group_position_remover
                    .active_group_position_iterator
                    .bitmap_group,
                expected_bitmap_group_1
            );

            remover.remove(ctx);

            // Outer index removed because group closed
            assert_eq!(remover.order_id_to_remove(), None);
            assert_eq!(remover.outer_index(), None);
            assert_eq!(
                remover.outer_index_remover.cached_outer_indices,
                vec![outer_index_0]
            );
            // Group position remains unchanged
            assert_eq!(
                remover.group_position().unwrap(),
                GroupPosition::from(&order_id_4)
            );

            // Best market price remains on order id 2
            assert_eq!(*remover.best_market_price, order_id_2.price_in_ticks);

            let mut expected_bitmap_group_1 = BitmapGroup::default();
            expected_bitmap_group_1.inner[0] = 0b0000_0000;
            expected_bitmap_group_1.inner[1] = 0b0000_0000;
            assert_eq!(
                remover
                    .group_position_remover
                    .active_group_position_iterator
                    .bitmap_group,
                expected_bitmap_group_1
            );
        }
    }

    mod inactive_orders {
        use super::*;

        #[test]
        fn test_lookup_inactive_ask_from_active_group() {
            let ctx = &mut ArbContext::new();
            let side = Side::Ask;

            let mut outer_index_count = 1;
            let outer_index_0 = OuterIndex::new(2);
            write_outer_indices(ctx, side, vec![outer_index_0]);

            let mut bitmap_group_0 = BitmapGroup::default();
            bitmap_group_0.inner[0] = 0b0000_0001;
            bitmap_group_0.write_to_slot(ctx, &outer_index_0);

            let order_id_0 = OrderId {
                price_in_ticks: Ticks::from_indices(outer_index_0, InnerIndex::new(0)),
                resting_order_index: RestingOrderIndex::new(0),
            };

            let inactive_order_id = OrderId {
                price_in_ticks: Ticks::from_indices(outer_index_0, InnerIndex::new(1)),
                resting_order_index: RestingOrderIndex::new(0),
            };

            let mut best_market_price = order_id_0.price_in_ticks;
            let mut remover =
                OrderLookupRemover::new(side, &mut best_market_price, &mut outer_index_count);

            assert_eq!(remover.find(ctx, inactive_order_id), false);
            assert_eq!(remover.order_id_to_remove().unwrap(), inactive_order_id);
            assert_eq!(
                remover.outer_index().unwrap(),
                inactive_order_id.price_in_ticks.outer_index()
            );
            assert_eq!(remover.outer_index_remover.unread_outer_indices(), 0);
            assert_eq!(remover.outer_index_remover.cached_outer_indices, vec![]);

            // No change in price
            assert_eq!(*remover.best_market_price, order_id_0.price_in_ticks);
        }

        #[test]
        fn test_lookup_ask_from_inactive_group() {
            let ctx = &mut ArbContext::new();
            let side = Side::Ask;

            let mut outer_index_count = 1;
            let outer_index_0 = OuterIndex::new(2);
            let outer_index_inactive = OuterIndex::new(1);
            write_outer_indices(ctx, side, vec![outer_index_0]);

            let mut bitmap_group_0 = BitmapGroup::default();
            bitmap_group_0.inner[0] = 0b0000_0001;
            bitmap_group_0.write_to_slot(ctx, &outer_index_0);

            let order_id_0 = OrderId {
                price_in_ticks: Ticks::from_indices(outer_index_0, InnerIndex::new(0)),
                resting_order_index: RestingOrderIndex::new(0),
            };

            let inactive_order_id = OrderId {
                price_in_ticks: Ticks::from_indices(outer_index_inactive, InnerIndex::new(0)),
                resting_order_index: RestingOrderIndex::new(0),
            };

            let mut best_market_price = order_id_0.price_in_ticks;
            let mut remover =
                OrderLookupRemover::new(side, &mut best_market_price, &mut outer_index_count);

            assert_eq!(remover.find(ctx, inactive_order_id), false);
            assert_eq!(remover.order_id_to_remove(), None);
            assert_eq!(remover.outer_index(), None);
            assert_eq!(
                remover.outer_index_remover.cached_outer_indices,
                vec![outer_index_0]
            );
            assert_eq!(remover.outer_index_remover.unread_outer_indices(), 0);
            assert_eq!(*remover.best_market_price, order_id_0.price_in_ticks);

            // Group position is None because no position was set
            assert_eq!(remover.group_position(), None);
        }

        // Special case- lookup beyond market price after sequential remove.
        // If order_price > best_market price, should we handle this in the remover
        // or in the manager?
        // The manager sets side to None if price is beyond best market price. In this
        // case the remover is never called. If sequential remover updates the best market
        // price, the side for the subsequent order id should be None and find() should
        // never be called.
        #[test]
        fn test_lookup_ask_beyond_best_market_price() {
            let ctx = &mut ArbContext::new();
            let side = Side::Ask;

            let mut outer_index_count = 1;
            let outer_index_0 = OuterIndex::new(2);
            write_outer_indices(ctx, side, vec![outer_index_0]);

            let mut bitmap_group_0 = BitmapGroup::default();
            bitmap_group_0.inner[0] = 0b0000_0001;
            bitmap_group_0.write_to_slot(ctx, &outer_index_0);

            let order_id_0 = OrderId {
                price_in_ticks: Ticks::from_indices(outer_index_0, InnerIndex::new(0)),
                resting_order_index: RestingOrderIndex::new(0),
            };

            let inactive_outer_index = OuterIndex::new(1);
            let inactive_order_id = OrderId {
                price_in_ticks: Ticks::from_indices(inactive_outer_index, InnerIndex::new(0)),
                resting_order_index: RestingOrderIndex::new(0),
            };

            let mut best_market_price = order_id_0.price_in_ticks;
            let mut remover =
                OrderLookupRemover::new(side, &mut best_market_price, &mut outer_index_count);

            assert_eq!(remover.find(ctx, inactive_order_id), false);

            // assert_eq!(remover.order_id().unwrap(), inactive_order_id);
            // assert_eq!(
            //     remover.outer_index().unwrap(),
            //     inactive_order_id.price_in_ticks.outer_index()
            // );
            // assert_eq!(remover.outer_index_remover.outer_index_count(), 0);
            // assert_eq!(remover.outer_index_remover.cached_outer_indices, vec![]);

            // // No change in price
            // assert_eq!(*remover.best_market_price, order_id_0.price_in_ticks);
        }

        // TODO test lookup in old group after sequential remover updates best market price
    }
}
