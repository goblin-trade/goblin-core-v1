use crate::{
    quantities::Ticks,
    state::{
        order::{group_position::GroupPosition, order_id::OrderId},
        InnerIndex, MarketState, Side, SlotStorage, TickIndices,
    },
};

use super::{bitmap_remover::BitmapRemover, index_list_remover::IndexListRemover};

/// Removes resting orders from slot. The resting order itself is not written, instead
/// we update the bitmaps and index list to mark the order as cleared.
///
/// This involves 3 updates
///
/// 1. Bitmap group- Clear the corresponding bit
/// 2. Index list- Remove outer index if the corresponding bitmap group is cleared
/// 3. Market state- Update the outer index count and best price
///
pub struct RestingOrderSearcherAndRemover {
    /// To turn off bits in bitmap groups
    pub bitmap_remover: BitmapRemover,

    /// To lookup and remove outer indices
    pub index_list_remover: IndexListRemover,
}

impl RestingOrderSearcherAndRemover {
    pub fn new(outer_index_count: u16, side: Side) -> Self {
        RestingOrderSearcherAndRemover {
            bitmap_remover: BitmapRemover::new(side),
            index_list_remover: IndexListRemover::new(side, outer_index_count),
        }
    }

    pub fn side(&self) -> Side {
        self.index_list_remover.side()
    }

    /// Checks whether an order is present at the given order ID.
    /// The bitmap group is updated if the outer index does not match.
    ///
    /// Externally ensure that order IDs move away from the centre
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
