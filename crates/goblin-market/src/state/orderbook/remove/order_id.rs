use crate::{
    quantities::Ticks,
    state::{
        order::{group_position::GroupPosition, order_id::OrderId},
        InnerIndex, MarketState, Side, SlotStorage, TickIndices,
    },
};

use super::{group_position::GroupPositionRemover, outer_index::OuterIndexRemover};

/// Removes resting orders from slot. The resting order itself is not written, instead
/// we update the bitmaps and index list to mark the order as cleared.
///
/// This involves 3 updates
///
/// 1. Bitmap group- Clear the corresponding bit
/// 2. Index list- Remove outer index if the corresponding bitmap group is cleared
/// 3. Market state- Update the outer index count and best price
///
pub struct OrderIdRemover {
    /// To turn off bits in bitmap groups
    pub bitmap_remover: GroupPositionRemover,

    /// To lookup and remove outer indices
    pub index_list_remover: OuterIndexRemover,
}

impl OrderIdRemover {
    pub fn new(outer_index_count: u16, side: Side) -> Self {
        OrderIdRemover {
            bitmap_remover: GroupPositionRemover::new(side),
            index_list_remover: OuterIndexRemover::new(side, outer_index_count),
        }
    }

    pub fn side(&self) -> Side {
        self.index_list_remover.side()
    }

    /// Checks whether an order is present at the given order ID.
    /// The bitmap group is updated if the outer index does not match.
    ///
    /// Externally ensure that
    /// * order IDs move away from the centre
    /// * we don't find order ids belonging to the opposite side
    ///
    /// # Arguments
    ///
    /// * `slot_storage`
    /// * `order_id`
    ///
    pub fn order_present(&mut self, slot_storage: &mut SlotStorage, order_id: OrderId) -> bool {
        let OrderId {
            price_in_ticks,
            resting_order_index,
        } = order_id;
        let TickIndices {
            outer_index,
            inner_index,
        } = price_in_ticks.to_indices();

        // Read the bitmap group and outer index corresponding to order_id
        if self.bitmap_remover.last_outer_index != Some(outer_index) {
            let outer_index_present = self
                .index_list_remover
                .find_outer_index(slot_storage, outer_index);

            if !outer_index_present {
                return false;
            }

            self.bitmap_remover
                .load_outer_index(slot_storage, outer_index);
        }

        // Now check in bitmap group
        return self.bitmap_remover.order_present(GroupPosition {
            inner_index,
            resting_order_index,
        });
    }

    /// Move one position down the index list and load the corresponding bitmap group
    ///
    /// Externally ensure that this is only called when we're on the outermost outer index.
    /// This way there is no `found_outer_index` to push to the cache.
    ///
    pub fn slide_outer_index_and_bitmap_group(&mut self, slot_storage: &mut SlotStorage) -> bool {
        self.index_list_remover.slide(slot_storage);
        if let Some(next_outer_index) = self.index_list_remover.cached_outer_index {
            self.bitmap_remover
                .load_outer_index(slot_storage, next_outer_index);

            return true;
        }
        false
    }

    /// Get the next best active price tick
    ///
    /// Externally ensure that this is only called when we're on the outermost outer index.
    /// This condition is necessary for self.slide()
    ///
    /// # Arguments
    ///
    /// * `slot_storage`
    ///
    /// * `starting_index` - Lookup from this inner index and onwards (inclusive)
    /// in the first bitmap group. Rest of the bitmap groups are searched from the edges.
    ///
    pub fn get_best_price(
        &mut self,
        slot_storage: &mut SlotStorage,
        mut starting_index: Option<InnerIndex>,
    ) -> Ticks {
        loop {
            if let Some(best_price) = self
                .bitmap_remover
                .get_best_price_in_current(starting_index)
            {
                return best_price;
            }

            let slide_success = self.slide_outer_index_and_bitmap_group(slot_storage);
            // Lookup from beginning in remaining bitmap groups
            starting_index = None;

            if !slide_success {
                // Return default values if the index list is exhausted
                return match self.side() {
                    Side::Bid => Ticks::ZERO,
                    Side::Ask => Ticks::MAX,
                };
            }
        }
    }

    /// Remove an order from the book and update the best price in market state
    ///
    /// This involves
    ///
    /// 1. Deactivating bit in bitmap group
    /// 2. Updating the best market price if the outermost tick was closed
    /// 2. Removing the outer index if the outermost bitmap group was closed
    ///
    /// Externally ensure that the order's outer index is loaded correctly. `order_present()`
    /// must be called before calling `remove_order()`. Since we're at the same bitmap group,
    /// we can simply deactivate the bit at `group_position`.
    ///
    pub fn remove_order(
        &mut self,
        slot_storage: &mut SlotStorage,
        market_state: &mut MarketState,
        order_id: OrderId,
    ) {
        let group_position = GroupPosition::from(&order_id);
        self.bitmap_remover.deactivate_in_current(group_position);

        // Remove cached outer index if the bitmap group was closed
        if !self.bitmap_remover.bitmap_group.is_active() {
            self.index_list_remover.remove_cached_index();
        }

        // TODO remove order ID. The last found order_id should be cached
        if order_id.price_in_ticks == market_state.best_price(self.side()) {
            // - Obtain and set new best price.
            // - If the outermost group was closed then this loads a new group.
            // - Look for active ticks at `order_id.price_in_ticks` and worse,
            // because we don't want to include active ticks on the same
            // bitmap belonging to the opposite side.
            let new_best_price =
                self.get_best_price(slot_storage, Some(order_id.price_in_ticks.inner_index()));
            market_state.set_best_price(self.side(), new_best_price);
        }
    }

    /// Write the prepared outer indices to slot and update outer index count in market state
    /// The last cached bitmap group pending a write is also written to slot
    ///
    /// No removal case- The internal function calls ensure that nothing is written to slot.
    ///
    /// Slot writes- bitmap_group, index list. Market state is only updated in memory.
    ///
    /// # Arguments
    ///
    /// * `slot_storage`
    ///
    pub fn write_prepared_indices(
        &mut self,
        slot_storage: &mut SlotStorage,
        market_state: &mut MarketState,
    ) {
        self.bitmap_remover.flush_bitmap_group(slot_storage);
        market_state
            .set_outer_index_length(self.side(), self.index_list_remover.index_list_length());
        self.index_list_remover.write_index_list(slot_storage);
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        quantities::QuoteLots,
        state::{
            bitmap_group::BitmapGroup, insert::outer_index, ListKey, ListSlot, OuterIndex,
            RestingOrderIndex, SlotActions, SlotKey,
        },
    };

    use super::*;

    fn enable_order_id(slot_storage: &mut SlotStorage, order_id: OrderId) {
        let OrderId {
            price_in_ticks,
            resting_order_index,
        } = order_id;
        let TickIndices {
            outer_index,
            inner_index,
        } = price_in_ticks.to_indices();

        let mut bitmap_group = BitmapGroup::new_from_slot(slot_storage, outer_index);
        let mut bitmap = bitmap_group.get_bitmap_mut(&inner_index);
        bitmap.activate(&resting_order_index);

        bitmap_group.write_to_slot(slot_storage, &outer_index);
    }

    #[test]
    fn test_search_and_remove_same_inner_index() {
        let mut slot_storage = SlotStorage::new();
        let side = Side::Ask;

        let outer_index_count = 1;
        let outer_index_0 = OuterIndex::new(1);

        // Write outer indices to slot
        let list_key = ListKey { index: 0, side };
        let mut list_slot = ListSlot::default();
        list_slot.set(0, outer_index_0);
        list_slot.write_to_slot(&mut slot_storage, &list_key);

        // Opposite side bit belonging to bids
        let order_id_bid = OrderId {
            price_in_ticks: Ticks::from_indices(outer_index_0, InnerIndex::new(0)),
            resting_order_index: RestingOrderIndex::new(0),
        };

        let order_id_0 = OrderId {
            price_in_ticks: Ticks::from_indices(outer_index_0, InnerIndex::new(2)),
            resting_order_index: RestingOrderIndex::new(0),
        };
        let order_id_1 = OrderId {
            price_in_ticks: Ticks::from_indices(outer_index_0, InnerIndex::new(2)),
            resting_order_index: RestingOrderIndex::new(7),
        };
        enable_order_id(&mut slot_storage, order_id_bid);
        enable_order_id(&mut slot_storage, order_id_0);
        enable_order_id(&mut slot_storage, order_id_1);

        let mut market_state = MarketState {
            collected_quote_lot_fees: QuoteLots::ZERO,
            unclaimed_quote_lot_fees: QuoteLots::ZERO,
            bids_outer_indices: 1,
            asks_outer_indices: 1,
            best_bid_price: order_id_bid.price_in_ticks,
            best_ask_price: order_id_0.price_in_ticks,
        };

        let mut remover = OrderIdRemover::new(outer_index_count, side);

        // 1. Search
        assert!(remover.order_present(&mut slot_storage, order_id_0));

        // 2. Remove
        remover.remove_order(&mut slot_storage, &mut market_state, order_id_0);

        // No change in opposite side price
        assert_eq!(market_state.best_bid_price, order_id_bid.price_in_ticks);

        // No change in best price because another order is present at the same tick
        assert_eq!(market_state.best_ask_price, order_id_1.price_in_ticks);

        // 3. Write list
        remover.write_prepared_indices(&mut slot_storage, &mut market_state);
        // No change because group is still active
        assert_eq!(market_state.asks_outer_indices, 1);

        // Check bitmap group
        let mut expected_bitmap_group = BitmapGroup::default();
        expected_bitmap_group.inner[0] = 0b00000001;
        expected_bitmap_group.inner[2] = 0b10000000;
        assert_eq!(remover.bitmap_remover.bitmap_group, expected_bitmap_group);
    }

    #[test]
    fn test_search_and_remove_all_cases() {
        let mut slot_storage = SlotStorage::new();
        let side = Side::Ask;

        let outer_index_count = 2;
        let outer_index_0 = OuterIndex::new(1);
        let outer_index_1 = OuterIndex::new(2);

        // Write outer indices to slot
        let list_key = ListKey { index: 0, side };
        let mut list_slot = ListSlot::default();
        list_slot.set(0, outer_index_0);
        list_slot.set(1, outer_index_1);
        list_slot.write_to_slot(&mut slot_storage, &list_key);

        let order_ids = vec![
            // Belongs to bids
            OrderId {
                price_in_ticks: Ticks::from_indices(outer_index_0, InnerIndex::new(0)),
                resting_order_index: RestingOrderIndex::new(0),
            },
            // Outermost ask
            OrderId {
                price_in_ticks: Ticks::from_indices(outer_index_0, InnerIndex::new(2)),
                resting_order_index: RestingOrderIndex::new(0),
            },
            // Same inner index case
            OrderId {
                price_in_ticks: Ticks::from_indices(outer_index_0, InnerIndex::new(2)),
                resting_order_index: RestingOrderIndex::new(7),
            },
            // Different inner index case
            OrderId {
                price_in_ticks: Ticks::from_indices(outer_index_0, InnerIndex::new(4)),
                resting_order_index: RestingOrderIndex::new(1),
            },
            // Different outer index case
            OrderId {
                price_in_ticks: Ticks::from_indices(outer_index_1, InnerIndex::new(0)),
                resting_order_index: RestingOrderIndex::new(0),
            },
        ];

        for order_id in &order_ids {
            enable_order_id(&mut slot_storage, *order_id);
        }

        let mut remover = OrderIdRemover::new(outer_index_count, side);
        assert!(remover.order_present(&mut slot_storage, *order_ids.get(1).unwrap()));
    }
}
