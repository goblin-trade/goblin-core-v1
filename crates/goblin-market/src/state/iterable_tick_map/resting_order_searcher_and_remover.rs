use crate::state::{
    InnerIndex, MarketState, OrderId, RestingOrderIndex, Side, SlotStorage, TickIndices,
};

use super::{BitmapRemover, IndexListRemover};

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
            bitmap_remover: BitmapRemover::new(),
            index_list_remover: IndexListRemover::new(side, outer_index_count),
        }
    }

    pub fn side(&self) -> Side {
        self.index_list_remover.side()
    }

    /// Checks whether an order is present at the given order ID
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

        // 1. Setup outer index and bitmap group in bitmap remover
        if self.bitmap_remover.last_outer_index != Some(outer_index) {
            // 2. Check whether outer index exists in index list
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

            // TODO update best price in market state
            // if order_id == market.best_order_id, find the next best order id
            // by looping through bitmap groups
            if market_state.best_price(self.side()) == order_id.price_in_ticks {}
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
