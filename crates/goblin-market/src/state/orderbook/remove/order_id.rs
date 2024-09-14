use crate::{
    quantities::Ticks,
    state::{
        order::{group_position::GroupPosition, order_id::OrderId},
        InnerIndex, MarketState, OuterIndex, Side, SlotStorage, TickIndices,
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
    /// To lookup and remove outer indices
    pub index_list_remover: OuterIndexRemover,

    /// To turn off bits in bitmap groups
    pub bitmap_remover: GroupPositionRemover,
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

    pub fn last_outer_index(&self) -> Option<OuterIndex> {
        self.bitmap_remover.last_outer_index
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
    pub fn find_order(&mut self, slot_storage: &mut SlotStorage, order_id: OrderId) -> bool {
        let OrderId {
            price_in_ticks,
            resting_order_index,
        } = order_id;
        let TickIndices {
            outer_index,
            inner_index,
        } = price_in_ticks.to_indices();

        // Read the bitmap group and outer index corresponding to order_id
        if self.last_outer_index() != Some(outer_index) {
            let outer_index_present = self
                .index_list_remover
                .find_outer_index(slot_storage, outer_index);

            if !outer_index_present {
                return false;
            }

            self.bitmap_remover
                .load_outer_index(slot_storage, outer_index);
        }

        let group_position = GroupPosition {
            inner_index,
            resting_order_index,
        };

        // Search group position in bitmap group
        let order_present = self.bitmap_remover.order_present(group_position);

        order_present
    }

    /// Remove the last searched order from the book, and update the
    /// best price in market state if the outermost tick closed
    ///
    /// # Arguments
    ///
    /// * `slot_storage`
    /// * `market_state`
    ///
    pub fn remove_order(&mut self, slot_storage: &mut SlotStorage, market_state: &mut MarketState) {
        if let Some(order_id) = self.bitmap_remover.last_searched_order_id() {
            // Deactivate group position in bitmap group
            let group_position = GroupPosition::from(&order_id);
            self.bitmap_remover
                .deactivate_last_searched_group_position();

            let side = self.side();
            let best_opposite_price = market_state.best_price(side.opposite());
            if self.bitmap_remover.is_inactive(best_opposite_price) {
                self.index_list_remover.remove_cached_index();
            }

            // Update best market price if the outermost tick was closed
            if order_id.price_in_ticks == market_state.best_price(side) {
                let new_best_price =
                    self.get_best_price(slot_storage, Some(group_position.inner_index));

                market_state.set_best_price(side, new_best_price);
            }
        }
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
            // Lookup from beginning in remaining bitmap groups
            starting_index = None;

            let slide_success = self.slide_outer_index_and_bitmap_group(slot_storage);
            if !slide_success {
                // Return default values if the index list is exhausted
                return match self.side() {
                    Side::Bid => Ticks::ZERO,
                    Side::Ask => Ticks::MAX,
                };
            }
        }
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
        // TODO avoid flush if empty
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
            bitmap_group::BitmapGroup, ListKey, ListSlot, OuterIndex, RestingOrderIndex,
            SlotActions,
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
        let found_0 = remover.find_order(&mut slot_storage, order_id_0);
        assert!(found_0);
        assert_eq!(
            remover.bitmap_remover.last_searched_order_id().unwrap(),
            order_id_0
        );

        // 2. Remove
        remover.remove_order(&mut slot_storage, &mut market_state);

        // Removing will clear last_searched_group_position
        assert!(remover
            .bitmap_remover
            .last_searched_group_position
            .is_none());
        assert!(remover.bitmap_remover.last_searched_order_id().is_none());

        // No change in opposite side price
        assert_eq!(market_state.best_bid_price, order_id_bid.price_in_ticks);

        // No change in best price because another order is present at the same tick
        assert_eq!(market_state.best_ask_price, order_id_1.price_in_ticks);

        // No change in outer index
        assert_eq!(
            remover.bitmap_remover.last_outer_index.unwrap(),
            outer_index_0
        );

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
    fn test_search_and_remove_same_outer_index() {
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
            price_in_ticks: Ticks::from_indices(outer_index_0, InnerIndex::new(3)),
            resting_order_index: RestingOrderIndex::new(0),
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
        let found_0 = remover.find_order(&mut slot_storage, order_id_0);
        assert!(found_0);
        assert_eq!(
            remover.bitmap_remover.last_searched_order_id().unwrap(),
            order_id_0
        );

        // 2. Remove
        remover.remove_order(&mut slot_storage, &mut market_state);
        assert!(remover
            .bitmap_remover
            .last_searched_group_position
            .is_none());
        assert!(remover.bitmap_remover.last_searched_order_id().is_none());

        // No change in opposite side price
        assert_eq!(market_state.best_bid_price, order_id_bid.price_in_ticks);

        // Best price changed
        assert_eq!(market_state.best_ask_price, order_id_1.price_in_ticks);

        // No change in outer index
        assert_eq!(
            remover.bitmap_remover.last_outer_index.unwrap(),
            outer_index_0
        );

        // 3. Write list
        remover.write_prepared_indices(&mut slot_storage, &mut market_state);
        // No change because group is still active
        assert_eq!(market_state.asks_outer_indices, 1);

        // Check bitmap group
        let mut expected_bitmap_group = BitmapGroup::default();
        expected_bitmap_group.inner[0] = 0b00000001;
        expected_bitmap_group.inner[3] = 0b00000001;
        assert_eq!(remover.bitmap_remover.bitmap_group, expected_bitmap_group);
    }

    #[test]
    fn test_search_and_remove_different_outer_index() {
        let mut slot_storage = SlotStorage::new();
        let side = Side::Ask;

        let outer_index_count = 2;
        let outer_index_0 = OuterIndex::new(1);
        let outer_index_1 = OuterIndex::new(2);

        // Write outer indices to slot
        let list_key = ListKey { index: 0, side };
        let mut list_slot = ListSlot::default();
        list_slot.set(0, outer_index_1);
        list_slot.set(1, outer_index_0); // smaller index is at end of the list
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
            price_in_ticks: Ticks::from_indices(outer_index_1, InnerIndex::new(0)),
            resting_order_index: RestingOrderIndex::new(0),
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
        let found_0 = remover.find_order(&mut slot_storage, order_id_0);
        assert!(found_0);
        assert_eq!(
            remover.bitmap_remover.last_searched_order_id().unwrap(),
            order_id_0
        );

        // 2. Remove
        remover.remove_order(&mut slot_storage, &mut market_state);
        assert!(remover
            .bitmap_remover
            .last_searched_group_position
            .is_none());
        assert!(remover.bitmap_remover.last_searched_order_id().is_none());

        // No change in opposite side price
        assert_eq!(market_state.best_bid_price, order_id_bid.price_in_ticks);

        // Best price changed. We are now in outer index 1
        assert_eq!(market_state.best_ask_price, order_id_1.price_in_ticks);

        // Outer index changed
        assert_eq!(
            remover.bitmap_remover.last_outer_index.unwrap(),
            outer_index_1
        );

        // 3. Write list
        remover.write_prepared_indices(&mut slot_storage, &mut market_state);
        // Outer index list size reduced by 1
        assert_eq!(market_state.asks_outer_indices, 1);

        // We are now on the bitmap for outer_index_1
        let mut expected_bitmap_group = BitmapGroup::default();
        expected_bitmap_group.inner[0] = 0b00000001;
        assert_eq!(remover.bitmap_remover.bitmap_group, expected_bitmap_group);
    }

    #[test]
    fn test_search_one_but_remove_another_in_same_inner_index() {
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
            resting_order_index: RestingOrderIndex::new(1),
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

        // 1. Search order_id_0
        let found_0 = remover.find_order(&mut slot_storage, order_id_0);
        assert!(found_0);
        assert_eq!(
            remover.bitmap_remover.last_searched_order_id().unwrap(),
            order_id_0
        );

        // 2. Search order_id_1
        let found_1 = remover.find_order(&mut slot_storage, order_id_1);
        assert!(found_1);
        assert_eq!(
            remover.bitmap_remover.last_searched_order_id().unwrap(),
            order_id_1
        );
        assert_eq!(
            remover.bitmap_remover.last_outer_index.unwrap(),
            outer_index_0
        );
        assert_eq!(
            remover.index_list_remover.cached_outer_index.unwrap(),
            outer_index_0
        );
        assert_eq!(remover.index_list_remover.cache, vec![]);

        // 3. Remove
        remover.remove_order(&mut slot_storage, &mut market_state);
        assert!(remover
            .bitmap_remover
            .last_searched_group_position
            .is_none());
        assert!(remover.bitmap_remover.last_searched_order_id().is_none());

        // No change in opposite side price
        assert_eq!(market_state.best_bid_price, order_id_bid.price_in_ticks);

        // Best price not changed
        assert_eq!(market_state.best_ask_price, order_id_0.price_in_ticks);

        // No change in outer index
        assert_eq!(
            remover.bitmap_remover.last_outer_index.unwrap(),
            outer_index_0
        );

        // 4. Write list
        remover.write_prepared_indices(&mut slot_storage, &mut market_state);
        // No change because group is still active
        assert_eq!(market_state.asks_outer_indices, 1);

        // Check bitmap group
        let mut expected_bitmap_group = BitmapGroup::default();
        expected_bitmap_group.inner[0] = 0b00000001;
        expected_bitmap_group.inner[2] = 0b00000001;
        assert_eq!(remover.bitmap_remover.bitmap_group, expected_bitmap_group);
    }

    #[test]
    fn test_search_one_but_remove_another_in_same_outer_index() {
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
            price_in_ticks: Ticks::from_indices(outer_index_0, InnerIndex::new(3)),
            resting_order_index: RestingOrderIndex::new(0),
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

        // 1. Search order_id_0
        let found_0 = remover.find_order(&mut slot_storage, order_id_0);
        assert!(found_0);
        assert_eq!(
            remover.bitmap_remover.last_searched_order_id().unwrap(),
            order_id_0
        );

        // 2. Search order_id_1
        let found_1 = remover.find_order(&mut slot_storage, order_id_1);
        assert!(found_1);
        assert_eq!(
            remover.bitmap_remover.last_searched_order_id().unwrap(),
            order_id_1
        );
        assert_eq!(
            remover.bitmap_remover.last_outer_index.unwrap(),
            outer_index_0
        );
        assert_eq!(
            remover.index_list_remover.cached_outer_index.unwrap(),
            outer_index_0
        );
        assert_eq!(remover.index_list_remover.cache, vec![]);

        // 3. Remove
        remover.remove_order(&mut slot_storage, &mut market_state);
        assert!(remover
            .bitmap_remover
            .last_searched_group_position
            .is_none());
        assert!(remover.bitmap_remover.last_searched_order_id().is_none());

        // No change in opposite side price
        assert_eq!(market_state.best_bid_price, order_id_bid.price_in_ticks);

        // Best price not changed
        assert_eq!(market_state.best_ask_price, order_id_0.price_in_ticks);

        // No change in outer index
        assert_eq!(
            remover.bitmap_remover.last_outer_index.unwrap(),
            outer_index_0
        );

        // 4. Write list
        remover.write_prepared_indices(&mut slot_storage, &mut market_state);
        // No change because group is still active
        assert_eq!(market_state.asks_outer_indices, 1);

        // Check bitmap group
        let mut expected_bitmap_group = BitmapGroup::default();
        expected_bitmap_group.inner[0] = 0b00000001;
        expected_bitmap_group.inner[2] = 0b00000001;
        assert_eq!(remover.bitmap_remover.bitmap_group, expected_bitmap_group);
    }

    #[test]
    fn test_search_one_but_remove_another_in_different_outer_index() {
        let mut slot_storage = SlotStorage::new();
        let side = Side::Ask;

        let outer_index_count = 2;
        let outer_index_0 = OuterIndex::new(1);
        let outer_index_1 = OuterIndex::new(2);

        // Write outer indices to slot
        let list_key = ListKey { index: 0, side };
        let mut list_slot = ListSlot::default();
        list_slot.set(0, outer_index_1);
        list_slot.set(1, outer_index_0); // smaller index is at end of the list
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
            price_in_ticks: Ticks::from_indices(outer_index_1, InnerIndex::new(0)),
            resting_order_index: RestingOrderIndex::new(0),
        };
        enable_order_id(&mut slot_storage, order_id_bid);
        enable_order_id(&mut slot_storage, order_id_0);
        enable_order_id(&mut slot_storage, order_id_1);

        let mut market_state = MarketState {
            collected_quote_lot_fees: QuoteLots::ZERO,
            unclaimed_quote_lot_fees: QuoteLots::ZERO,
            bids_outer_indices: 1,
            asks_outer_indices: outer_index_count,
            best_bid_price: order_id_bid.price_in_ticks,
            best_ask_price: order_id_0.price_in_ticks,
        };

        let mut remover = OrderIdRemover::new(outer_index_count, side);

        // 1. Search order_id_0
        let found_0 = remover.find_order(&mut slot_storage, order_id_0);
        assert!(found_0);
        assert_eq!(
            remover.bitmap_remover.last_searched_order_id().unwrap(),
            order_id_0
        );

        // 2. Search order_id_1
        let found_1 = remover.find_order(&mut slot_storage, order_id_1);
        assert!(found_1);
        assert_eq!(
            remover.bitmap_remover.last_searched_order_id().unwrap(),
            order_id_1
        );
        assert_eq!(
            remover.bitmap_remover.last_outer_index.unwrap(),
            outer_index_1
        );
        assert_eq!(
            remover.index_list_remover.cached_outer_index.unwrap(),
            outer_index_1
        );
        assert_eq!(remover.index_list_remover.cache, vec![outer_index_0]);

        // 3. Remove
        remover.remove_order(&mut slot_storage, &mut market_state);
        assert!(remover
            .bitmap_remover
            .last_searched_group_position
            .is_none());
        assert!(remover.bitmap_remover.last_searched_order_id().is_none());

        // No change in opposite side price
        assert_eq!(market_state.best_bid_price, order_id_bid.price_in_ticks);

        // Best price not changed
        assert_eq!(market_state.best_ask_price, order_id_0.price_in_ticks);

        // Outer index changed
        assert_eq!(
            remover.bitmap_remover.last_outer_index.unwrap(),
            outer_index_1
        );

        let read_bitmap_group_1 = BitmapGroup::new_from_slot(&slot_storage, outer_index_1);
        println!("read_bitmap_group_1 {:?}", read_bitmap_group_1);

        // 4. Write list
        remover.write_prepared_indices(&mut slot_storage, &mut market_state);
        // Changed because outer_index_0 was closed
        assert_eq!(market_state.asks_outer_indices, 1);

        // Check bitmap group
        let mut expected_bitmap_group_0 = BitmapGroup::default();
        expected_bitmap_group_0.inner[0] = 0b00000001;
        expected_bitmap_group_0.inner[2] = 0b00000001;

        let read_bitmap_group_0 = BitmapGroup::new_from_slot(&slot_storage, outer_index_0);
        assert_eq!(read_bitmap_group_0, expected_bitmap_group_0);

        let expected_bitmap_group_1 = BitmapGroup::default();
        assert_eq!(remover.bitmap_remover.bitmap_group, expected_bitmap_group_1);

        // bitmap group 1 is not written to slot. Slot will give the previous value
        let mut expected_bitmap_group_1_written = BitmapGroup::default();
        expected_bitmap_group_1_written.inner[0] = 0b00000001;
        let read_bitmap_group_1 = BitmapGroup::new_from_slot(&slot_storage, outer_index_1);
        assert_eq!(read_bitmap_group_1, expected_bitmap_group_1_written);
    }

    // TODO test_search_one_but_remove_another()
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
        assert!(remover.find_order(&mut slot_storage, *order_ids.get(1).unwrap()));
    }
}
