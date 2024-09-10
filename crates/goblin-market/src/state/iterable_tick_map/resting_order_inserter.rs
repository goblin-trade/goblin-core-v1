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

        self.index_list_inserter.write_index_list(slot_storage);
    }
}

#[cfg(test)]
mod tests {
    use stylus_sdk::alloy_primitives::Address;

    use crate::{
        quantities::{BaseLots, QuoteLots, Ticks, WrapperU64},
        state::{
            bitmap_group::BitmapGroup, ListKey, ListSlot, OuterIndex, RestingOrderIndex,
            SlotActions, TickIndices,
        },
    };

    use super::*;

    #[test]
    fn insert_single_order_on_empty_index_list() {
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

        let mut inserter = RestingOrderInserter::new(side, market_state.outer_index_count(side));

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

        assert_eq!(vec![outer_index], inserter.index_list_inserter.cache);

        let mut read_bitmap_group = BitmapGroup::new_from_slot(slot_storage, outer_index);
        assert_eq!(BitmapGroup::default(), read_bitmap_group);

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

    #[test]
    fn insert_two_orders_at_same_price_on_empty_index_list() {
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

        let order_id_0 = OrderId {
            price_in_ticks,
            resting_order_index: RestingOrderIndex::new(0),
        };
        let order_id_1 = OrderId {
            price_in_ticks,
            resting_order_index: RestingOrderIndex::new(1),
        };

        let resting_order_0 = SlotRestingOrder {
            trader_address: Address::default(),
            num_base_lots: BaseLots::new(100),
            track_block: false,
            last_valid_block_or_unix_timestamp_in_seconds: 0,
        };

        let resting_order_1 = SlotRestingOrder {
            trader_address: Address::default(),
            num_base_lots: BaseLots::new(200),
            track_block: false,
            last_valid_block_or_unix_timestamp_in_seconds: 0,
        };

        let mut inserter = RestingOrderInserter::new(side, market_state.outer_index_count(side));

        inserter
            .insert_resting_order(
                slot_storage,
                &mut market_state,
                &resting_order_0,
                &order_id_0,
            )
            .unwrap();

        inserter
            .insert_resting_order(
                slot_storage,
                &mut market_state,
                &resting_order_1,
                &order_id_1,
            )
            .unwrap();

        // 1. Check updated market state
        assert_eq!(market_state.best_bid_price, price_in_ticks);
        assert_eq!(market_state.bids_outer_indices, 0); // No change yet

        // 2. Check resting order and market state from slot
        let read_resting_order_0 = SlotRestingOrder::new_from_slot(slot_storage, order_id_0);
        assert_eq!(resting_order_0, read_resting_order_0);

        let read_resting_order_1 = SlotRestingOrder::new_from_slot(slot_storage, order_id_1);
        assert_eq!(resting_order_1, read_resting_order_1);

        // 3. Check cached values
        assert_eq!(
            outer_index,
            inserter.bitmap_inserter.last_outer_index.unwrap()
        );

        let mut expected_bitmap_group = BitmapGroup::default();
        expected_bitmap_group.inner[inner_index.as_usize()] = 0b00000011; // Two resting orders
        assert_eq!(expected_bitmap_group, inserter.bitmap_inserter.bitmap_group);

        // Outer index is common. There should only be a single value
        assert_eq!(vec![outer_index], inserter.index_list_inserter.cache);

        let mut read_bitmap_group = BitmapGroup::new_from_slot(slot_storage, outer_index);
        assert_eq!(BitmapGroup::default(), read_bitmap_group);

        // 3. Check values after writing indices
        inserter.write_prepared_indices(slot_storage, &mut market_state);

        assert_eq!(market_state.bids_outer_indices, 1);

        read_bitmap_group = BitmapGroup::new_from_slot(slot_storage, outer_index);
        assert_eq!(expected_bitmap_group, read_bitmap_group);

        let list_key = ListKey { index: 0, side };
        let list_slot = ListSlot::new_from_slot(slot_storage, list_key);
        let mut expected_list_slot = ListSlot::default();
        expected_list_slot.set(0, outer_index);
        assert_eq!(expected_list_slot, list_slot);
    }

    #[test]
    fn insert_two_orders_at_different_ticks_on_same_bitmap_group_on_empty_index_list() {
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

        // Order closer to the centre is inserted first
        let price_in_ticks_0 = Ticks::new(1);
        let price_in_ticks_1 = Ticks::new(0);

        let outer_index = OuterIndex::new(0);
        let inner_index_0 = price_in_ticks_0.inner_index();
        let inner_index_1 = price_in_ticks_1.inner_index();

        let order_id_0 = OrderId {
            price_in_ticks: price_in_ticks_0,
            resting_order_index: RestingOrderIndex::new(0),
        };
        let order_id_1 = OrderId {
            price_in_ticks: price_in_ticks_1,
            resting_order_index: RestingOrderIndex::new(0),
        };

        let resting_order_0 = SlotRestingOrder {
            trader_address: Address::default(),
            num_base_lots: BaseLots::new(100),
            track_block: false,
            last_valid_block_or_unix_timestamp_in_seconds: 0,
        };

        let resting_order_1 = SlotRestingOrder {
            trader_address: Address::default(),
            num_base_lots: BaseLots::new(200),
            track_block: false,
            last_valid_block_or_unix_timestamp_in_seconds: 0,
        };

        let mut inserter = RestingOrderInserter::new(side, market_state.outer_index_count(side));

        inserter
            .insert_resting_order(
                slot_storage,
                &mut market_state,
                &resting_order_0,
                &order_id_0,
            )
            .unwrap();

        inserter
            .insert_resting_order(
                slot_storage,
                &mut market_state,
                &resting_order_1,
                &order_id_1,
            )
            .unwrap();

        // 1. Check updated market state
        assert_eq!(market_state.best_bid_price, price_in_ticks_0);
        assert_eq!(market_state.bids_outer_indices, 0); // No change yet

        // 2. Check resting order and market state from slot
        let read_resting_order_0 = SlotRestingOrder::new_from_slot(slot_storage, order_id_0);
        assert_eq!(resting_order_0, read_resting_order_0);

        let read_resting_order_1 = SlotRestingOrder::new_from_slot(slot_storage, order_id_1);
        assert_eq!(resting_order_1, read_resting_order_1);

        // 3. Check cached values
        assert_eq!(
            outer_index,
            inserter.bitmap_inserter.last_outer_index.unwrap()
        );

        let mut expected_bitmap_group = BitmapGroup::default();
        expected_bitmap_group.inner[inner_index_0.as_usize()] = 0b00000001;
        expected_bitmap_group.inner[inner_index_1.as_usize()] = 0b00000001;
        assert_eq!(expected_bitmap_group, inserter.bitmap_inserter.bitmap_group);

        // Outer index is common. There should only be a single value
        assert_eq!(vec![outer_index], inserter.index_list_inserter.cache);

        let mut read_bitmap_group = BitmapGroup::new_from_slot(slot_storage, outer_index);
        assert_eq!(BitmapGroup::default(), read_bitmap_group);

        // 3. Check values after writing indices
        inserter.write_prepared_indices(slot_storage, &mut market_state);

        assert_eq!(market_state.bids_outer_indices, 1);

        read_bitmap_group = BitmapGroup::new_from_slot(slot_storage, outer_index);
        assert_eq!(expected_bitmap_group, read_bitmap_group);

        let list_key = ListKey { index: 0, side };
        let list_slot = ListSlot::new_from_slot(slot_storage, list_key);
        let mut expected_list_slot = ListSlot::default();
        expected_list_slot.set(0, outer_index);
        assert_eq!(expected_list_slot, list_slot);
    }

    #[test]
    fn insert_two_orders_at_bitmap_groups_on_empty_index_list() {
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

        // Order closer to the centre is inserted first
        let price_in_ticks_0 = Ticks::new(32);
        let price_in_ticks_1 = Ticks::new(0);

        let TickIndices {
            outer_index: outer_index_0,
            inner_index: inner_index_0,
        } = price_in_ticks_0.to_indices();
        let TickIndices {
            outer_index: outer_index_1,
            inner_index: inner_index_1,
        } = price_in_ticks_1.to_indices();

        let order_id_0 = OrderId {
            price_in_ticks: price_in_ticks_0,
            resting_order_index: RestingOrderIndex::new(0),
        };
        let order_id_1 = OrderId {
            price_in_ticks: price_in_ticks_1,
            resting_order_index: RestingOrderIndex::new(0),
        };

        let resting_order_0 = SlotRestingOrder {
            trader_address: Address::default(),
            num_base_lots: BaseLots::new(100),
            track_block: false,
            last_valid_block_or_unix_timestamp_in_seconds: 0,
        };

        let resting_order_1 = SlotRestingOrder {
            trader_address: Address::default(),
            num_base_lots: BaseLots::new(200),
            track_block: false,
            last_valid_block_or_unix_timestamp_in_seconds: 0,
        };

        let mut inserter = RestingOrderInserter::new(side, market_state.outer_index_count(side));

        inserter
            .insert_resting_order(
                slot_storage,
                &mut market_state,
                &resting_order_0,
                &order_id_0,
            )
            .unwrap();

        inserter
            .insert_resting_order(
                slot_storage,
                &mut market_state,
                &resting_order_1,
                &order_id_1,
            )
            .unwrap();

        // 1. Check updated market state
        assert_eq!(market_state.best_bid_price, price_in_ticks_0);
        assert_eq!(market_state.bids_outer_indices, 0); // No change yet

        // 2. Check resting order and market state from slot
        let read_resting_order_0 = SlotRestingOrder::new_from_slot(slot_storage, order_id_0);
        assert_eq!(resting_order_0, read_resting_order_0);

        let read_resting_order_1 = SlotRestingOrder::new_from_slot(slot_storage, order_id_1);
        assert_eq!(resting_order_1, read_resting_order_1);

        // 3. Check cached values
        assert_eq!(
            outer_index_1,
            inserter.bitmap_inserter.last_outer_index.unwrap()
        );

        let mut expected_bitmap_group_0 = BitmapGroup::default();
        expected_bitmap_group_0.inner[inner_index_0.as_usize()] = 0b00000001;

        let mut expected_bitmap_group_1 = BitmapGroup::default();
        expected_bitmap_group_1.inner[inner_index_1.as_usize()] = 0b00000001;

        // The second bitmap is cached. The first is already written to slot
        assert_eq!(
            expected_bitmap_group_1,
            inserter.bitmap_inserter.bitmap_group
        );

        let read_bitmap_group_0 = BitmapGroup::new_from_slot(slot_storage, outer_index_0);
        assert_eq!(expected_bitmap_group_0, read_bitmap_group_0);

        let mut read_bitmap_group_1 = BitmapGroup::new_from_slot(slot_storage, outer_index_1);
        assert_eq!(BitmapGroup::default(), read_bitmap_group_1);

        assert_eq!(
            vec![outer_index_0, outer_index_1],
            inserter.index_list_inserter.cache
        );

        // 3. Check values after writing indices
        inserter.write_prepared_indices(slot_storage, &mut market_state);

        assert_eq!(market_state.bids_outer_indices, 2);

        read_bitmap_group_1 = BitmapGroup::new_from_slot(slot_storage, outer_index_1);
        assert_eq!(expected_bitmap_group_1, read_bitmap_group_1);

        let list_key = ListKey { index: 0, side };
        let list_slot = ListSlot::new_from_slot(slot_storage, list_key);
        let mut expected_list_slot = ListSlot::default();
        expected_list_slot.set(0, outer_index_1);
        expected_list_slot.set(1, outer_index_0);
        assert_eq!(expected_list_slot, list_slot);
    }

    // insert on non-empty values
    // 1. order present on same bitmap group
    // 2. on different bitmap group

    #[test]
    fn insert_single_order_on_non_empty_index_list_on_same_bitmap_group() {
        let slot_storage = &mut SlotStorage::new();
        let side = Side::Bid;

        let price_in_ticks_0 = Ticks::new(1);
        let TickIndices {
            outer_index: outer_index_0,
            inner_index: inner_index_0,
        } = price_in_ticks_0.to_indices();

        // Pre test setup- push outer_index_0 to list and activate a bit at outer_index_0
        let list_key = ListKey { index: 0, side };
        let mut list_slot = ListSlot::default();
        list_slot.set(0, outer_index_0);
        list_slot.write_to_slot(slot_storage, &list_key);

        let mut bitmap_group_0 = BitmapGroup::default();
        bitmap_group_0.inner[inner_index_0.as_usize()] = 0b00000001;
        bitmap_group_0.write_to_slot(slot_storage, &outer_index_0);

        // No need to insert resting order for price_in_ticks_0

        let mut market_state = MarketState {
            collected_quote_lot_fees: QuoteLots::ZERO,
            unclaimed_quote_lot_fees: QuoteLots::ZERO,
            bids_outer_indices: 1,
            asks_outer_indices: 0,
            best_bid_price: price_in_ticks_0,
            best_ask_price: Ticks::ZERO,
        };

        let price_in_ticks_1 = Ticks::new(10);
        let TickIndices {
            outer_index: outer_index_1,
            inner_index: inner_index_1,
        } = price_in_ticks_1.to_indices();

        let order_id_1 = OrderId {
            price_in_ticks: price_in_ticks_1,
            resting_order_index: RestingOrderIndex::new(0),
        };

        let resting_order = SlotRestingOrder {
            trader_address: Address::default(),
            num_base_lots: BaseLots::new(100),
            track_block: false,
            last_valid_block_or_unix_timestamp_in_seconds: 0,
        };

        let mut inserter = RestingOrderInserter::new(side, market_state.outer_index_count(side));

        inserter
            .insert_resting_order(slot_storage, &mut market_state, &resting_order, &order_id_1)
            .unwrap();

        // 1. Check updated market state
        assert_eq!(market_state.best_bid_price, price_in_ticks_1); // Best price changed
        assert_eq!(market_state.bids_outer_indices, 1); // No change

        // 2. Check resting order and market state from slot
        let read_resting_order = SlotRestingOrder::new_from_slot(slot_storage, order_id_1);
        assert_eq!(resting_order, read_resting_order);

        // 3. Check cached values
        assert_eq!(
            outer_index_1,
            inserter.bitmap_inserter.last_outer_index.unwrap()
        );
        let mut expected_bitmap_group = BitmapGroup::default();
        expected_bitmap_group.inner[inner_index_0.as_usize()] = 0b00000001;
        expected_bitmap_group.inner[inner_index_1.as_usize()] = 0b00000001;

        assert_eq!(expected_bitmap_group, inserter.bitmap_inserter.bitmap_group);

        assert_eq!(vec![outer_index_1], inserter.index_list_inserter.cache);

        let mut read_bitmap_group = BitmapGroup::new_from_slot(slot_storage, outer_index_1);
        assert_eq!(bitmap_group_0, read_bitmap_group);

        // 3. Check values after writing indices
        inserter.write_prepared_indices(slot_storage, &mut market_state);

        assert_eq!(market_state.bids_outer_indices, 1); // No change since outer index is common

        read_bitmap_group = BitmapGroup::new_from_slot(slot_storage, outer_index_1);
        assert_eq!(expected_bitmap_group, read_bitmap_group);

        let read_list_slot = ListSlot::new_from_slot(slot_storage, list_key);
        assert_eq!(list_slot, read_list_slot); // No change since outer index is common
    }

    #[test]
    fn insert_single_order_on_non_empty_index_list_on_different_bitmap_groups() {
        let slot_storage = &mut SlotStorage::new();
        let side = Side::Bid;

        let price_in_ticks_0 = Ticks::new(1);
        let TickIndices {
            outer_index: outer_index_0,
            inner_index: inner_index_0,
        } = price_in_ticks_0.to_indices();

        // Pre test setup- push outer_index_0 to list and activate a bit at outer_index_0
        let list_key = ListKey { index: 0, side };
        let mut list_slot = ListSlot::default();
        list_slot.set(0, outer_index_0);
        list_slot.write_to_slot(slot_storage, &list_key);

        let mut bitmap_group_0 = BitmapGroup::default();
        bitmap_group_0.inner[inner_index_0.as_usize()] = 0b00000001;
        bitmap_group_0.write_to_slot(slot_storage, &outer_index_0);

        // No need to insert resting order for price_in_ticks_0

        let mut market_state = MarketState {
            collected_quote_lot_fees: QuoteLots::ZERO,
            unclaimed_quote_lot_fees: QuoteLots::ZERO,
            bids_outer_indices: 1,
            asks_outer_indices: 0,
            best_bid_price: price_in_ticks_0,
            best_ask_price: Ticks::ZERO,
        };

        let price_in_ticks_1 = Ticks::new(32);
        let TickIndices {
            outer_index: outer_index_1,
            inner_index: inner_index_1,
        } = price_in_ticks_1.to_indices();

        let order_id_1 = OrderId {
            price_in_ticks: price_in_ticks_1,
            resting_order_index: RestingOrderIndex::new(0),
        };

        let resting_order = SlotRestingOrder {
            trader_address: Address::default(),
            num_base_lots: BaseLots::new(100),
            track_block: false,
            last_valid_block_or_unix_timestamp_in_seconds: 0,
        };

        let mut inserter = RestingOrderInserter::new(side, market_state.outer_index_count(side));

        inserter
            .insert_resting_order(slot_storage, &mut market_state, &resting_order, &order_id_1)
            .unwrap();

        // 1. Check updated market state
        assert_eq!(market_state.best_bid_price, price_in_ticks_1); // Best price changed
        assert_eq!(market_state.bids_outer_indices, 1); // No change yet

        // 2. Check resting order and market state from slot
        let read_resting_order = SlotRestingOrder::new_from_slot(slot_storage, order_id_1);
        assert_eq!(resting_order, read_resting_order);

        // 3. Check cached values
        assert_eq!(
            outer_index_1,
            inserter.bitmap_inserter.last_outer_index.unwrap()
        );

        assert_eq!(
            vec![outer_index_1, outer_index_0],
            inserter.index_list_inserter.cache
        );

        // No change in bitmap group 0
        let mut read_bitmap_group_0 = BitmapGroup::new_from_slot(slot_storage, outer_index_0);
        assert_eq!(bitmap_group_0, read_bitmap_group_0);

        // Bitmap group 1 is cached, but not written yet
        let mut expected_bitmap_group_1 = BitmapGroup::default();
        expected_bitmap_group_1.inner[inner_index_1.as_usize()] = 0b00000001;
        assert_eq!(
            expected_bitmap_group_1,
            inserter.bitmap_inserter.bitmap_group
        );
        let mut read_bitmap_group_1 = BitmapGroup::new_from_slot(slot_storage, outer_index_1);
        assert_eq!(BitmapGroup::default(), read_bitmap_group_1);

        // 3. Check values after writing indices
        inserter.write_prepared_indices(slot_storage, &mut market_state);
        assert_eq!(market_state.bids_outer_indices, 2);

        // No change in bitmap group 0
        read_bitmap_group_0 = BitmapGroup::new_from_slot(slot_storage, outer_index_0);
        assert_eq!(bitmap_group_0, read_bitmap_group_0);

        // bitmap group 1 is written
        read_bitmap_group_1 = BitmapGroup::new_from_slot(slot_storage, outer_index_1);
        assert_eq!(expected_bitmap_group_1, read_bitmap_group_1);

        // outer_index_1 added to list
        list_slot.set(1, outer_index_1);
        list_slot.write_to_slot(slot_storage, &list_key);

        let read_list_slot = ListSlot::new_from_slot(slot_storage, list_key);
        assert_eq!(list_slot, read_list_slot);
    }

    #[test]
    fn insert_two_orders_on_non_empty_index_list_on_different_bitmap_groups() {
        let slot_storage = &mut SlotStorage::new();
        let side = Side::Bid;

        let price_in_ticks_0 = Ticks::new(1);
        let TickIndices {
            outer_index: outer_index_0,
            inner_index: inner_index_0,
        } = price_in_ticks_0.to_indices();

        // Pre test setup- push outer_index_0 to list and activate a bit at outer_index_0
        let list_key = ListKey { index: 0, side };
        let mut list_slot = ListSlot::default();
        list_slot.set(0, outer_index_0);
        list_slot.write_to_slot(slot_storage, &list_key);

        let mut bitmap_group_0 = BitmapGroup::default();
        bitmap_group_0.inner[inner_index_0.as_usize()] = 0b00000001;
        bitmap_group_0.write_to_slot(slot_storage, &outer_index_0);

        // No need to insert resting order for price_in_ticks_0

        let mut market_state = MarketState {
            collected_quote_lot_fees: QuoteLots::ZERO,
            unclaimed_quote_lot_fees: QuoteLots::ZERO,
            bids_outer_indices: 1,
            asks_outer_indices: 0,
            best_bid_price: price_in_ticks_0,
            best_ask_price: Ticks::ZERO,
        };

        // Higher price inserted first for bids
        let price_in_ticks_1 = Ticks::new(32);
        let price_in_ticks_2 = Ticks::new(64);

        let TickIndices {
            outer_index: outer_index_1,
            inner_index: inner_index_1,
        } = price_in_ticks_1.to_indices();

        let TickIndices {
            outer_index: outer_index_2,
            inner_index: inner_index_2,
        } = price_in_ticks_2.to_indices();

        let order_id_1 = OrderId {
            price_in_ticks: price_in_ticks_1,
            resting_order_index: RestingOrderIndex::new(0),
        };

        let order_id_2 = OrderId {
            price_in_ticks: price_in_ticks_2,
            resting_order_index: RestingOrderIndex::new(0),
        };

        let resting_order = SlotRestingOrder {
            trader_address: Address::default(),
            num_base_lots: BaseLots::new(100),
            track_block: false,
            last_valid_block_or_unix_timestamp_in_seconds: 0,
        };

        let mut inserter = RestingOrderInserter::new(side, market_state.outer_index_count(side));

        inserter
            .insert_resting_order(slot_storage, &mut market_state, &resting_order, &order_id_1)
            .unwrap();

        inserter
            .insert_resting_order(slot_storage, &mut market_state, &resting_order, &order_id_2)
            .unwrap();

        // 1. Check updated market state
        assert_eq!(market_state.best_bid_price, price_in_ticks_1); // Best price changed
        assert_eq!(market_state.bids_outer_indices, 1); // No change yet

        // 2. Check resting order and market state from slot
        let read_resting_order_1 = SlotRestingOrder::new_from_slot(slot_storage, order_id_1);
        assert_eq!(resting_order, read_resting_order_1);
        let read_resting_order_2 = SlotRestingOrder::new_from_slot(slot_storage, order_id_2);
        assert_eq!(resting_order, read_resting_order_2);

        // 3. Check cached values
        assert_eq!(
            outer_index_2,
            inserter.bitmap_inserter.last_outer_index.unwrap()
        );

        assert_eq!(
            vec![outer_index_1, outer_index_2, outer_index_0], // [2, 1, 0]
            inserter.index_list_inserter.cache
        );

        // No change in bitmap group 0
        let mut read_bitmap_group_0 = BitmapGroup::new_from_slot(slot_storage, outer_index_0);
        assert_eq!(bitmap_group_0, read_bitmap_group_0);

        // Bitmap group 1 is written
        let mut expected_bitmap_group_1 = BitmapGroup::default();
        expected_bitmap_group_1.inner[inner_index_1.as_usize()] = 0b00000001;

        let mut read_bitmap_group_1 = BitmapGroup::new_from_slot(slot_storage, outer_index_1);
        assert_eq!(expected_bitmap_group_1, read_bitmap_group_1);

        // Bitmap group 2 is still cached
        let mut expected_bitmap_group_2 = BitmapGroup::default();
        expected_bitmap_group_2.inner[inner_index_2.as_usize()] = 0b00000001;
        assert_eq!(
            expected_bitmap_group_2,
            inserter.bitmap_inserter.bitmap_group
        );

        // 3. Check values after writing indices
        inserter.write_prepared_indices(slot_storage, &mut market_state);
        assert_eq!(market_state.bids_outer_indices, 3);

        // No change in bitmap group 0 and 1
        read_bitmap_group_0 = BitmapGroup::new_from_slot(slot_storage, outer_index_0);
        assert_eq!(bitmap_group_0, read_bitmap_group_0);
        read_bitmap_group_1 = BitmapGroup::new_from_slot(slot_storage, outer_index_1);
        assert_eq!(expected_bitmap_group_1, read_bitmap_group_1);

        // bitmap group 2 is written
        let read_bitmap_group_2 = BitmapGroup::new_from_slot(slot_storage, outer_index_2);
        assert_eq!(expected_bitmap_group_2, read_bitmap_group_2);

        // outer_index_2 and outer_index_1 and added to list
        list_slot.set(1, outer_index_2);
        list_slot.set(2, outer_index_1);
        list_slot.write_to_slot(slot_storage, &list_key); // [outer_index_0, outer_index_2, outer_index_1]

        let read_list_slot = ListSlot::new_from_slot(slot_storage, list_key);
        assert_eq!(list_slot, read_list_slot);
    }
}
