use crate::{
    program::GoblinResult,
    state::{MarketState, OrderId, Side, SlotRestingOrder, SlotStorage},
};

use super::{BitmapInserter, IndexListInserter};

/// Inserts resting orders to slot
///
/// This involves 4 updates
///
/// 1. Market state- Update best price and outer index count
/// 2. Resting order- Save to slot
/// 3. Index list- Insert outer index if not present
/// 4. Bitmap group- Flip bit corresponding to the order
///
/// It caches the last read bitmap group and its outer index to minimize slot writes.
/// Multiple updates to a bitmap group are batched.
///
pub struct RestingOrderInserter {
    /// Index list inserter- to insert outer indices in index lists and for writing them to slot
    pub index_list_inserter: IndexListInserter,

    /// Bitmap inserter- to activate bits in bitmap groups and writing them to slot
    pub bitmap_inserter: BitmapInserter,
}

impl RestingOrderInserter {
    pub fn new(side: Side, outer_index_count: u16) -> Self {
        RestingOrderInserter {
            index_list_inserter: IndexListInserter::new(side, outer_index_count),
            bitmap_inserter: BitmapInserter::new(),
        }
    }

    pub fn side(&self) -> Side {
        self.index_list_inserter.side()
    }

    /// Write a resting order to slot and prepare for insertion of its outer index
    /// in the index list
    ///
    /// Slot writes- resting_order, bitmap_group
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
        //
        // Since the first element is closest to the centre, we only need
        // to check the first element against the current best price.
        //
        // Outer index length in market state is updated in write_prepared_indices()
        //
        if self.index_list_inserter.cache.len() == 0 {
            market_state.try_set_best_price(self.side(), order_id.price_in_ticks);
        }

        // 2. Write resting order to slot
        resting_order.write_to_slot(slot_storage, &order_id)?;

        // 3. Try to insert outer index in list
        // Find whether it was inserted or whether it was already present
        let outer_index = order_id.price_in_ticks.outer_index();
        let needs_insertion = self.index_list_inserter.prepare(slot_storage, outer_index);

        // 4. Update bitmap
        self.bitmap_inserter
            .activate(slot_storage, order_id, needs_insertion);

        Ok(())
    }

    /// Write the prepared outer indices to slot and update outer index count in market state
    /// The last cached bitmap group pending a write is also written to slot
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
        self.bitmap_inserter.write_last_bitmap_group(slot_storage);

        market_state.set_outer_index_length(
            self.side(),
            self.index_list_inserter.index_list_reader.outer_index_count
                + self.index_list_inserter.cache.len() as u16,
        );

        self.index_list_inserter
            .write_prepared_indices(slot_storage);
    }
}

#[cfg(test)]
mod tests {
    use stylus_sdk::alloy_primitives::Address;

    use crate::{
        quantities::{BaseLots, QuoteLots, Ticks, WrapperU64},
        state::{BitmapGroup, ListKey, ListSlot, RestingOrderIndex, SlotActions, TickIndices},
    };

    use super::*;

    #[test]
    fn test_insert_first_bid_on_empty_index_list() {
        let slot_storage = &mut SlotStorage::new();
        let side = Side::Bid;

        let mut market_state = MarketState {
            collected_quote_lot_fees: QuoteLots::ZERO,
            unclaimed_quote_lot_fees: QuoteLots::ZERO,
            bids_outer_indices: 0,
            asks_outer_indices: 0,
            best_bid_price: Ticks::ZERO,
            best_ask_price: Ticks::ZERO,
        };

        let price_in_ticks = Ticks::new(10);
        let TickIndices {
            outer_index,
            inner_index,
        } = price_in_ticks.to_indices();

        let order_id = OrderId {
            price_in_ticks,
            resting_order_index: RestingOrderIndex::new(0),
        };

        let resting_order = SlotRestingOrder {
            trader_address: Address::default(),
            num_base_lots: BaseLots::new(100),
            track_block: false,
            last_valid_block_or_unix_timestamp_in_seconds: 0,
        };

        let mut inserter = RestingOrderInserter::new(side, market_state.outer_index_length(side));

        inserter
            .insert_resting_order(slot_storage, &mut market_state, &resting_order, &order_id)
            .unwrap();

        // 1. Check updated market state
        assert_eq!(market_state.best_bid_price, price_in_ticks);
        assert_eq!(market_state.bids_outer_indices, 0); // No change

        // 2. Check resting order and market state from slot
        let read_resting_order = SlotRestingOrder::new_from_slot(slot_storage, order_id);
        assert_eq!(resting_order, read_resting_order);

        // 3. Check cached values
        assert_eq!(
            outer_index,
            inserter.bitmap_inserter.last_outer_index.unwrap()
        );
        let bitmap = inserter
            .bitmap_inserter
            .bitmap_group
            .get_bitmap(&inner_index);
        assert_eq!(0b00000001, *bitmap.inner);

        assert_eq!(1, inserter.index_list_inserter.cache.len());
        assert_eq!(
            outer_index,
            *inserter.index_list_inserter.cache.last().unwrap()
        );

        let mut read_bitmap_group = BitmapGroup::new_from_slot(slot_storage, outer_index);
        assert_eq!(read_bitmap_group, BitmapGroup { inner: [0; 32] });

        // 3. Check values after writing indices
        inserter.write_prepared_indices(slot_storage, &mut market_state);

        assert_eq!(market_state.bids_outer_indices, 1);

        read_bitmap_group = BitmapGroup::new_from_slot(slot_storage, outer_index);
        let read_bitmap = read_bitmap_group.get_bitmap(&inner_index);
        assert_eq!(0b00000001, *read_bitmap.inner);

        let mut expected_bitmap_group = BitmapGroup { inner: [0u8; 32] };
        expected_bitmap_group.inner[inner_index.as_usize()] = 0b00000001;
        assert_eq!(expected_bitmap_group, read_bitmap_group);

        let list_key = ListKey { index: 0, side };
        let list_slot = ListSlot::new_from_slot(slot_storage, list_key);
        let mut expected_list_slot = ListSlot::default();
        expected_list_slot.set(0, outer_index);
        assert_eq!(expected_list_slot, list_slot);
    }

    // TODO separate struct to insert bits in bitmap group
    // Test cases- same tick, different tick in same group, different groups
    #[test]
    fn test_insert_multiple_bids_at_same_price_on_empty_index_list() {}

    #[test]
    fn test_insert_multiple_bids_in_same_bitmap_group_on_empty_index_list() {}
}
