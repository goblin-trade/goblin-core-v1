use crate::state::{
    InnerIndex, MarketState, OrderId, RestingOrderIndex, Side, SlotStorage, TickIndices,
};

use super::{BitmapRemover, GroupPosition, IndexListRemover};

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
                .set_outer_index(slot_storage, outer_index);
        }

        // Now check in bitmap group
        return self
            .bitmap_remover
            .order_present(inner_index, resting_order_index);
    }

    /// Move one position down the index list and load the corresponding bitmap group
    ///
    /// TODO This is a special case of slide used to find next best price. It is only called when
    /// we're on the outermost bitmap group.
    pub fn slide(&mut self, slot_storage: &mut SlotStorage) -> bool {
        if let Some(next_outer_index) = self.index_list_remover.slide(slot_storage) {
            self.bitmap_remover
                .set_outer_index(slot_storage, next_outer_index);

            return true;
        }
        false
    }

    /// Find the next active bit across all active bitmaps
    ///
    /// TODO this is a special case to find next best price. All of the traversed
    /// outer indices should be closed.
    ///
    pub fn get_next_active_bit_in_all_groups(
        &mut self,
        slot_storage: &mut SlotStorage,
        mut position_to_exclude: Option<GroupPosition>,
    ) -> Option<OrderId> {
        loop {
            if let Some(order_id) = self.bitmap_remover.get_next_active_bit(position_to_exclude) {
                return Some(order_id);
            }
            position_to_exclude = None;

            if !self.slide(slot_storage) {
                // If slide fails then we have reached end of the list
                return None;
            }
        }
    }

    /// Remove an order from the current bitmap group
    pub fn deactivate_in_current(&mut self, group_position: GroupPosition) {
        self.bitmap_remover.deactivate_in_current(group_position);
    }

    pub fn get_best_inner_index_in_current(&self) {}

    // TODO function to get best inner index across all groups
    // Find best price in current. If not found, slide
    pub fn get_best_price_in_current(&self) {}

    /// Marks a resting order as removed
    ///
    /// This involves
    ///
    /// 1. Deactivating its bit in bitmap group
    /// 2. Removing the outer index if the bitmap group was turned off
    /// 3. Updating best market price
    ///
    /// Externally ensure that order_ids are in correct order and that order_present()
    /// was called before remove_order() for the given order ID
    ///
    pub fn remove_order(
        &mut self,
        slot_storage: &mut SlotStorage,
        order_id: OrderId,
        market_state: &mut MarketState,
    ) {
        let OrderId {
            price_in_ticks,
            resting_order_index,
        } = order_id;
        let TickIndices {
            outer_index,
            inner_index,
        } = price_in_ticks.to_indices();

        self.bitmap_remover.deactivate(slot_storage, &order_id);

        if !self.bitmap_remover.bitmap_group.is_active() {
            self.index_list_remover.remove(slot_storage, outer_index);
        }

        // TODO move this to a separate function.
        // This shouldn't be part of this struct

        // if order_id == market.best_order_id, find the next best order id
        // by looping through bitmap groups
        if market_state.best_price(self.side()) == order_id.price_in_ticks {
            // Currently we can only find next best bit within a group
            // We need to find the 'next best bit' across groups. We need to traverse
            // outer indices.
            // We can't simply move up or down bitmaps. We may have to traverse
            // the index list to obtain outer indices of active bitmaps
            //
            // If the outermost value was removed then we need to loop from beginning
            // of the index list. This means reading cached values from index_list_remover
            //
            // RestingOrderSearcherAndRemover already combines these two. The outer index
            // within BitmapRemover is the index we searched
            //
            // Property- if an order is removed on the outermost tick, then its outermost
            // index is present in `found_outer_index` and the index cache is empty.
            // Need a slide function to move down one outer index and its corresponding bitmap
            // slide()
            // - Commit bitmap group. It should be empty.
            // - Read next outer index. Drop `found_outer_index` because this group was
            // closed.
            //
            // Question- do we write closed bitmap groups to slot with a placeholder,
            // or we simply drop the outer index?
            // In order to know whether there's a placeholder, we still need to read
            // outer index. Therefore placeholder is unnecessary. If a non-empty bitmap
            // group is being closed, its slot data is guaranteed to be non-empty.
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
        self.bitmap_remover.write_last_bitmap_group(slot_storage);
        market_state
            .set_outer_index_length(self.side(), self.index_list_remover.index_list_length());
        self.index_list_remover.write_prepared_indices(slot_storage);
    }
}
