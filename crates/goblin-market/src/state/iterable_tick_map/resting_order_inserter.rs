use crate::{
    program::GoblinResult,
    state::{MarketState, OrderId, OuterIndex, Side, SlotRestingOrder, SlotStorage, TickIndices},
};

use super::{BitmapGroup, IndexListInserter};

/// Inserts resting orders to slot
///
/// This involves 4 state updates
///
/// 1. Market state- Update best price
/// 2. Resting order- Save to slot
/// 3. Index list- Insert outer index if not present
/// 4. Bitmap group- Flip bit corresponding to the order
///
/// It caches the last read bitmap group and its outer index to minimize slot writes.
/// Multiple updates to a bitmap group are batched.
///
pub struct RestingOrderInserter {
    /// Index list inserter
    pub index_list_inserter: IndexListInserter,

    /// The current bitmap group pending a write. This allows us to perform multiple
    /// updates in a bitmap group with a single slot load. This value is written to slot
    /// when a new outer index is encountered.
    pub bitmap_group: BitmapGroup,

    /// Outer index corresponding to `bitmap_group`
    pub last_outer_index: Option<OuterIndex>,
}

impl RestingOrderInserter {
    pub fn new(side: Side, outer_index_count: u16) -> Self {
        RestingOrderInserter {
            index_list_inserter: IndexListInserter::new(side, outer_index_count),
            bitmap_group: BitmapGroup::default(),
            last_outer_index: None,
        }
    }

    pub fn side(&self) -> Side {
        self.index_list_inserter.side()
    }

    /// Write cached bitmap group to slot
    /// This should be called when the outer index changes during looping, and when the loop is complete
    pub fn write_last_bitmap_group(&self, slot_storage: &mut SlotStorage) {
        if let Some(last_index) = self.last_outer_index {
            self.bitmap_group.write_to_slot(slot_storage, &last_index);
        }
    }

    /// Write a resting order to slot and prepare for insertion of its outer index
    /// in the index list
    ///
    /// # Arguments
    ///
    /// * `slot_storage`
    /// * `market_state`
    /// * `resting_order`
    /// * `order_id`
    ///
    pub fn insert_resting_order(
        &mut self,
        slot_storage: &mut SlotStorage,
        market_state: &mut MarketState,
        resting_order: &SlotRestingOrder,
        order_id: &OrderId,
    ) -> GoblinResult<()> {
        // 1. Update market state
        // Optimization- since the first element is closest to the centre, we only need
        // to check the first element against the current best price.
        if self.index_list_inserter.cache.len() == 0 {
            // Update best market price
            if self.side() == Side::Bid && order_id.price_in_ticks > market_state.best_bid_price {
                market_state.best_bid_price = order_id.price_in_ticks;
            }

            if self.side() == Side::Ask && order_id.price_in_ticks < market_state.best_ask_price {
                market_state.best_ask_price = order_id.price_in_ticks;
            }
        }

        // 2. Write resting order to slot
        resting_order.write_to_slot(slot_storage, &order_id)?;

        let TickIndices {
            outer_index,
            inner_index,
        } = order_id.price_in_ticks.to_indices();

        // 3. Try to insert outer index in list
        // Find whether it was inserted or whether it was already present
        let needs_insertion = self.index_list_inserter.prepare(slot_storage, outer_index);

        // 4. Load bitmap group
        // Outer index changed or first iteration- load bitmap group
        if self.last_outer_index != Some(outer_index) {
            // Outer index changed. Write bitmap group belonging to the old index to slot.
            self.write_last_bitmap_group(slot_storage);

            self.bitmap_group = if needs_insertion {
                BitmapGroup::default()
            } else {
                BitmapGroup::new_from_slot(slot_storage, outer_index)
            };

            self.last_outer_index = Some(outer_index);
        }

        // 5. Flip tick in bitmap
        let mut bitmap = self.bitmap_group.get_bitmap_mut(&inner_index);
        bitmap.activate(&order_id.resting_order_index);

        Ok(())
    }

    /// Write the prepared outer indices to slot
    /// The last cached bitmap group pending a write is also written to slot
    ///
    /// # Arguments
    ///
    /// * `slot_storage`
    ///
    pub fn write_prepared_indices(&mut self, slot_storage: &mut SlotStorage) {
        self.write_last_bitmap_group(slot_storage);
        self.index_list_inserter
            .write_prepared_indices(slot_storage);
    }
}
