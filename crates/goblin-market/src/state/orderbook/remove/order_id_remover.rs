use crate::{
    quantities::Ticks,
    state::{
        order::{
            group_position::{self, GroupPosition},
            order_id::OrderId,
        },
        ArbContext, InnerIndex, MarketPrices, MarketPricesForSide, MarketState, OuterIndex,
        RestingOrderIndex, Side, TickIndices,
    },
};

use super::{group_position_remover::GroupPositionRemover, outer_index_remover::OuterIndexRemover};

/// Lookup and remove order ids from the orderbook. Successive order ids must move away
/// from the centre, i.e. ascending order for asks and descending for bids.
///
/// Removal involves deactivating the order id bit in the bitmap group.
/// The cleared resting order is not written to slot.
///
/// This involves 3 updates
///
/// 1. Bitmap group- Clear the corresponding bit
/// 2. Index list- Remove outer index if the corresponding bitmap group is cleared
/// 3. Market state- Update the outer index count and best price
///
/// # Gas optimizations and garbage values
///
/// Slot writes are minimized where possible, resulting in garbage values.
/// Ensure that garbage values are not read from slot and are discarded.
///
/// 1. If a `BitmapGroup` closes, do not write the cleared group to slot. Simply remove
/// the outer index from the index list.
///
/// 2. If a `ListSlot` in the index list clears, do not write the cleared value to slot.
/// Simply decrement the `outer_index_count` in market state.
///
/// 3. If the most recent removal causes `best_market_price` to update, do not write the
/// updated `BitmapGroup` to slot. Instead simply update the `best_market_price` in
/// `MarketPrice`. This price will be used to clear garbage bits during insertions.
///
pub struct OrderIdRemover {
    /// To lookup and remove outer indices
    pub outer_index_remover: OuterIndexRemover,

    /// To lookup and deactivate bits in bitmap groups
    pub group_position_remover: GroupPositionRemover,

    /// Whether the bitmap group was updated in memory and is pending a write.
    /// write_last_bitmap_group() should write to slot only if this is true.
    pub pending_write: bool,
}

impl OrderIdRemover {
    pub fn new(outer_index_count: u16, side: Side) -> Self {
        OrderIdRemover {
            group_position_remover: GroupPositionRemover::new(side),
            outer_index_remover: OuterIndexRemover::new(side, outer_index_count),
            pending_write: false,
        }
    }

    /// Create a new order id remover that is initialized to iterate through
    /// active order ids
    ///
    /// Initialization involves
    /// - Reading the outermost bitmap group
    /// - Setting the initial position to the best tick
    ///
    /// Garbage bits are not cleared
    pub fn new_for_matching(
        ctx: &mut ArbContext,
        side: Side,
        outer_index_count: u16,
        market_prices: &MarketPrices,
    ) -> Option<Self> {
        if outer_index_count == 0 {
            return None;
        }
        let mut remover = OrderIdRemover::new(outer_index_count, side);

        // Load outermost outer index
        remover.slide_outer_index_and_load_bitmap_group(ctx);

        // Clear garbage bits
        remover.try_clear_garbage_bits(market_prices);

        let best_market_price = market_prices.best_market_price(side);

        // Set initial group position- this needs to be set after sliding
        let initial_group_position = GroupPosition {
            inner_index: best_market_price.inner_index(),
            resting_order_index: RestingOrderIndex::ZERO,
        };
        remover.group_position_remover.group_position = initial_group_position;

        Some(remover)
    }

    pub fn side(&self) -> Side {
        self.outer_index_remover.side()
    }

    pub fn last_outer_index(&self) -> Option<OuterIndex> {
        self.outer_index_remover.cached_outer_index
    }

    pub fn group_position(&self) -> GroupPosition {
        self.group_position_remover.group_position
    }

    pub fn inner_index(&self) -> InnerIndex {
        self.group_position().inner_index
    }

    // Externally ensure that order index is present
    pub fn outer_index_unchecked(&self) -> OuterIndex {
        let outer_index = self.last_outer_index();
        debug_assert!(outer_index.is_some());

        unsafe { outer_index.unwrap_unchecked() }
    }

    pub fn price(&self) -> Ticks {
        let inner_index = self.group_position().inner_index;
        let outer_index = self.outer_index_unchecked();

        Ticks::from_indices(outer_index, inner_index)
    }

    // Externally ensure that order index is present
    pub fn order_id(&self) -> OrderId {
        let group_position = self.group_position_remover.group_position;

        OrderId::from_group_position(group_position, self.outer_index_unchecked())
    }

    pub fn flush_bitmap_group(&mut self, ctx: &mut ArbContext) {
        if !self.pending_write {
            return;
        }
        // If pending_write is true then outer_index is guaranteed to be present
        let outer_index = self.outer_index_unchecked();
        self.group_position_remover
            .bitmap_group
            .write_to_slot(ctx, &outer_index);
        self.pending_write = false;
    }

    /// Clear garbage bits in the current bitmap group
    ///
    /// # Arguments
    ///
    /// * `best_market_prices`- Best market prices before performing any remove operation
    pub fn try_clear_garbage_bits(&mut self, best_market_prices: &MarketPrices) {
        let outer_index = self.outer_index_unchecked();
        self.group_position_remover
            .bitmap_group
            .clear_garbage_bits(outer_index, best_market_prices);
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
    /// * `ctx`
    /// * `order_id`
    /// * `market_prices` - The initial market prices before removing any order.
    /// They're used to clear garbage bits if we're on the outermost group.
    ///
    pub fn order_id_is_active(
        &mut self,
        ctx: &mut ArbContext,
        market_prices: &MarketPrices,
        order_id: OrderId,
    ) -> bool {
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
            let outer_index_present = self.outer_index_remover.find_outer_index(ctx, outer_index);

            if !outer_index_present {
                return false;
            }

            self.flush_bitmap_group(ctx);
            self.group_position_remover
                .load_outer_index(ctx, outer_index);

            self.try_clear_garbage_bits(market_prices);
        }

        let group_position = GroupPosition {
            inner_index,
            resting_order_index,
        };

        // Search group position in bitmap group
        let order_present = self
            .group_position_remover
            .paginate_and_check_if_active(group_position);

        order_present
    }

    /// Get the best active order ID
    ///
    /// Externally ensure that the struct is initialized with new_for_matching() to setup
    /// outer index and starting group position and for clearing garbage bits
    pub fn best_active_order_id(&mut self, ctx: &mut ArbContext) -> Option<OrderId> {
        loop {
            if self
                .group_position_remover
                .try_traverse_to_best_active_position()
            {
                return Some(self.order_id());
            }

            // Slide to the next outer index if the current one is traversed
            let slide_success = self.slide_outer_index_and_load_bitmap_group(ctx);
            if !slide_success {
                return None;
            }
        }
    }

    pub fn best_active_price(&mut self, ctx: &mut ArbContext) -> Option<Ticks> {
        self.best_active_order_id(ctx)
            .map(|order_id| order_id.price_in_ticks)
    }

    /// Remove the last searched order from the book, and update the
    /// best price in market state if the outermost tick closed
    ///
    /// Externally ensure this is not called if no order id was searched
    ///
    /// # Arguments
    ///
    /// * `ctx`
    /// * `market_state`
    ///
    pub fn remove_order(&mut self, ctx: &mut ArbContext, market_state: &mut MarketState) {
        let outer_index = self.outer_index_unchecked();
        let inner_index = self.inner_index();

        let MarketPricesForSide {
            best_market_price,
            best_opposite_price,
        } = market_state.get_prices_for_side(self.side());

        // Perform group, index list and market state transitions together

        // 1. Deactivate in group
        self.group_position_remover.deactivate();

        let inner_index_deactivated = !self
            .group_position_remover
            .is_inner_index_active(inner_index);

        let market_price_deactivated = self.price() == best_market_price && inner_index_deactivated;

        let group_deactivated = self
            .group_position_remover
            .is_group_inactive(best_opposite_price, outer_index);

        self.pending_write = !(market_price_deactivated || group_deactivated);

        if group_deactivated {
            self.outer_index_remover.remove_cached_index();
        }

        if market_price_deactivated {
            let new_best_price = self.best_active_price(ctx);
            market_state.try_update_best_price(self.side(), new_best_price);
        }
    }

    /// Move one position down the index list and load the corresponding bitmap group
    ///
    /// Externally ensure that this is only called when we're on the outermost outer index.
    /// This way there is no `found_outer_index` to push to the cache.
    ///
    pub fn slide_outer_index_and_load_bitmap_group(&mut self, ctx: &mut ArbContext) -> bool {
        self.outer_index_remover.slide(ctx);
        if let Some(next_outer_index) = self.outer_index_remover.cached_outer_index {
            self.group_position_remover
                .load_outer_index(ctx, next_outer_index);

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
    /// * `ctx`
    ///
    pub fn write_prepared_indices(&mut self, ctx: &mut ArbContext, market_state: &mut MarketState) {
        self.flush_bitmap_group(ctx);

        market_state
            .set_outer_index_length(self.side(), self.outer_index_remover.index_list_length());
        self.outer_index_remover.write_index_list(ctx);
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        quantities::QuoteLots,
        state::{
            bitmap_group::BitmapGroup, ContextActions, InnerIndex, ListKey, ListSlot, OuterIndex,
            RestingOrderIndex,
        },
    };

    use super::*;

    fn enable_order_id(ctx: &mut ArbContext, order_id: OrderId) {
        let OrderId {
            price_in_ticks,
            resting_order_index,
        } = order_id;
        let TickIndices {
            outer_index,
            inner_index,
        } = price_in_ticks.to_indices();

        let mut bitmap_group = BitmapGroup::new_from_slot(ctx, outer_index);
        let mut bitmap = bitmap_group.get_bitmap_mut(&inner_index);
        bitmap.activate(&resting_order_index);

        bitmap_group.write_to_slot(ctx, &outer_index);
    }

    mod search_and_remove {
        use super::*;

        #[test]
        fn test_search_and_remove_same_inner_index() {
            let mut ctx = ArbContext::new();
            let side = Side::Ask;

            let outer_index_count = 1;
            let outer_index_0 = OuterIndex::new(1);

            // Write outer indices to slot
            let list_key = ListKey { index: 0, side };
            let mut list_slot = ListSlot::default();
            list_slot.set(0, outer_index_0);
            list_slot.write_to_slot(&mut ctx, &list_key);

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
            enable_order_id(&mut ctx, order_id_bid);
            enable_order_id(&mut ctx, order_id_0);
            enable_order_id(&mut ctx, order_id_1);

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
            let found_0 =
                remover.order_id_is_active(&mut ctx, &market_state.get_prices(), order_id_0);
            assert!(found_0);
            assert_eq!(remover.order_id(), order_id_0);

            // 2. Remove
            remover.remove_order(&mut ctx, &mut market_state);

            // pending_write is true because
            // - We're in the outermost group
            // - Outermost tick did not close
            assert!(remover.pending_write);

            // No change in opposite side price
            assert_eq!(market_state.best_bid_price, order_id_bid.price_in_ticks);

            // No change in best price because another order is present at the same tick
            assert_eq!(market_state.best_ask_price, order_id_1.price_in_ticks);

            // No change in outer index
            assert_eq!(remover.outer_index_unchecked(), outer_index_0);

            // 3. Write list
            remover.write_prepared_indices(&mut ctx, &mut market_state);
            // No change because group is still active
            assert_eq!(market_state.asks_outer_indices, 1);

            // Check bitmap group
            // Group updated because `pending_write` was true
            let mut expected_bitmap_group = BitmapGroup::default();
            expected_bitmap_group.inner[0] = 0b00000001;
            expected_bitmap_group.inner[2] = 0b10000000;
            assert_eq!(
                remover.group_position_remover.bitmap_group,
                expected_bitmap_group
            );

            let read_bitmap_group = BitmapGroup::new_from_slot(&ctx, outer_index_0);
            assert_eq!(read_bitmap_group, expected_bitmap_group);
        }

        #[test]
        fn test_search_and_remove_same_outer_index() {
            let mut ctx = ArbContext::new();
            let side = Side::Ask;

            let outer_index_count = 1;
            let outer_index_0 = OuterIndex::new(1);

            // Write outer indices to slot
            let list_key = ListKey { index: 0, side };
            let mut list_slot = ListSlot::default();
            list_slot.set(0, outer_index_0);
            list_slot.write_to_slot(&mut ctx, &list_key);

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
            enable_order_id(&mut ctx, order_id_bid);
            enable_order_id(&mut ctx, order_id_0);
            enable_order_id(&mut ctx, order_id_1);

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
            let found_0 =
                remover.order_id_is_active(&mut ctx, &market_state.get_prices(), order_id_0);
            assert!(found_0);
            assert_eq!(remover.order_id(), order_id_0);

            // 2. Remove
            remover.remove_order(&mut ctx, &mut market_state);

            // Pending write is false because the best market price changed
            assert_eq!(remover.pending_write, false);

            // No change in opposite side price
            assert_eq!(market_state.best_bid_price, order_id_bid.price_in_ticks);

            // Best price changed
            assert_eq!(market_state.best_ask_price, order_id_1.price_in_ticks);

            // No change in outer index
            assert_eq!(remover.outer_index_unchecked(), outer_index_0);

            // 3. Write list
            remover.write_prepared_indices(&mut ctx, &mut market_state);
            // No change because group is still active
            assert_eq!(market_state.asks_outer_indices, 1);

            // Check bitmap group
            let mut expected_cached_bitmap_group = BitmapGroup::default();
            expected_cached_bitmap_group.inner[0] = 0b00000001;
            expected_cached_bitmap_group.inner[3] = 0b00000001;
            assert_eq!(
                remover.group_position_remover.bitmap_group,
                expected_cached_bitmap_group
            );

            // Bitmap group not written because `pending_write` is false. No change in value
            let read_bitmap_group = BitmapGroup::new_from_slot(&ctx, outer_index_0);
            let mut expected_read_bitmap_group = BitmapGroup::default();
            expected_read_bitmap_group.inner[0] = 0b00000001;
            expected_read_bitmap_group.inner[2] = 0b00000001;
            expected_read_bitmap_group.inner[3] = 0b00000001;

            assert_eq!(read_bitmap_group, expected_read_bitmap_group);
        }

        #[test]
        fn test_search_and_remove_same_outer_index_non_outermost_value() {
            let mut ctx = ArbContext::new();
            let side = Side::Ask;

            let outer_index_count = 1;
            let outer_index_0 = OuterIndex::new(1);

            // Write outer indices to slot
            let list_key = ListKey { index: 0, side };
            let mut list_slot = ListSlot::default();
            list_slot.set(0, outer_index_0);
            list_slot.write_to_slot(&mut ctx, &list_key);

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
            enable_order_id(&mut ctx, order_id_bid);
            enable_order_id(&mut ctx, order_id_0);
            enable_order_id(&mut ctx, order_id_1);

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
            let found_1 =
                remover.order_id_is_active(&mut ctx, &market_state.get_prices(), order_id_1);
            assert!(found_1);
            assert_eq!(remover.order_id(), order_id_1);

            // 2. Remove
            remover.remove_order(&mut ctx, &mut market_state);

            // Pending write is true because the best market price did not change
            assert_eq!(remover.pending_write, true);

            // No change in opposite side price
            assert_eq!(market_state.best_bid_price, order_id_bid.price_in_ticks);

            // Best price did not change
            assert_eq!(market_state.best_ask_price, order_id_0.price_in_ticks);

            // No change in outer index
            assert_eq!(remover.outer_index_unchecked(), outer_index_0);

            // 3. Write list
            remover.write_prepared_indices(&mut ctx, &mut market_state);
            // No change because group is still active
            assert_eq!(market_state.asks_outer_indices, 1);

            // Check bitmap group
            let mut expected_bitmap_group = BitmapGroup::default();
            expected_bitmap_group.inner[0] = 0b00000001;
            expected_bitmap_group.inner[2] = 0b00000001;
            assert_eq!(
                remover.group_position_remover.bitmap_group,
                expected_bitmap_group
            );
            let read_bitmap_group = BitmapGroup::new_from_slot(&ctx, outer_index_0);
            assert_eq!(read_bitmap_group, expected_bitmap_group);
        }

        #[test]
        fn test_search_and_remove_different_outer_index() {
            let mut ctx = ArbContext::new();
            let side = Side::Ask;

            let outer_index_count = 2;
            let outer_index_0 = OuterIndex::new(1);
            let outer_index_1 = OuterIndex::new(2);

            // Write outer indices to slot
            let list_key = ListKey { index: 0, side };
            let mut list_slot = ListSlot::default();
            list_slot.set(0, outer_index_1);
            list_slot.set(1, outer_index_0); // smaller index is at end of the list
            list_slot.write_to_slot(&mut ctx, &list_key);

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
            enable_order_id(&mut ctx, order_id_bid);
            enable_order_id(&mut ctx, order_id_0);
            enable_order_id(&mut ctx, order_id_1);

            let mut market_state = MarketState {
                collected_quote_lot_fees: QuoteLots::ZERO,
                unclaimed_quote_lot_fees: QuoteLots::ZERO,
                bids_outer_indices: 1,
                asks_outer_indices: 2,
                best_bid_price: order_id_bid.price_in_ticks,
                best_ask_price: order_id_0.price_in_ticks,
            };

            let mut remover = OrderIdRemover::new(outer_index_count, side);

            // 1. Search
            let found_0 =
                remover.order_id_is_active(&mut ctx, &market_state.get_prices(), order_id_0);
            assert!(found_0);
            assert_eq!(remover.order_id(), order_id_0);

            println!(
                "outer index before removal {:?}",
                remover.last_outer_index()
            );

            // 2. Remove
            // problem when updating market price?
            remover.remove_order(&mut ctx, &mut market_state);

            // No pending write because group was cleared
            assert_eq!(remover.pending_write, false);

            // No change in opposite side price
            assert_eq!(market_state.best_bid_price, order_id_bid.price_in_ticks);

            // Best price changed. We are now in outer index 1
            assert_eq!(market_state.best_ask_price, order_id_1.price_in_ticks);

            // Outer index changed
            assert_eq!(remover.outer_index_unchecked(), outer_index_1);

            // 3. Write list
            remover.write_prepared_indices(&mut ctx, &mut market_state);
            // Outer index list size reduced by 1
            assert_eq!(market_state.asks_outer_indices, 1);

            // bitmap_group_0 was cleared, so no slot update
            let mut expected_bitmap_group_0 = BitmapGroup::default();
            expected_bitmap_group_0.inner[0] = 0b00000001;
            expected_bitmap_group_0.inner[2] = 0b00000001;
            let read_bitmap_group_0 = BitmapGroup::new_from_slot(&ctx, outer_index_0);
            assert_eq!(read_bitmap_group_0, expected_bitmap_group_0);

            // We are now on the bitmap for outer_index_1
            let mut expected_cached_bitmap_group_1 = BitmapGroup::default();
            expected_cached_bitmap_group_1.inner[0] = 0b00000001;
            assert_eq!(
                remover.group_position_remover.bitmap_group,
                expected_cached_bitmap_group_1
            );
        }

        #[test]
        fn test_search_one_but_remove_another_in_same_inner_index() {
            let mut ctx = ArbContext::new();
            let side = Side::Ask;

            let outer_index_count = 1;
            let outer_index_0 = OuterIndex::new(1);

            // Write outer indices to slot
            let list_key = ListKey { index: 0, side };
            let mut list_slot = ListSlot::default();
            list_slot.set(0, outer_index_0);
            list_slot.write_to_slot(&mut ctx, &list_key);

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
            enable_order_id(&mut ctx, order_id_bid);
            enable_order_id(&mut ctx, order_id_0);
            enable_order_id(&mut ctx, order_id_1);

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
            let found_0 =
                remover.order_id_is_active(&mut ctx, &market_state.get_prices(), order_id_0);
            assert!(found_0);
            assert_eq!(remover.order_id(), order_id_0);

            // 2. Search order_id_1
            let found_1 =
                remover.order_id_is_active(&mut ctx, &market_state.get_prices(), order_id_1);
            assert!(found_1);
            assert_eq!(remover.order_id(), order_id_1);
            assert_eq!(remover.outer_index_unchecked(), outer_index_0);
            assert_eq!(
                remover.outer_index_remover.cached_outer_index.unwrap(),
                outer_index_0
            );
            assert_eq!(remover.outer_index_remover.cache, vec![]);

            // 3. Remove
            remover.remove_order(&mut ctx, &mut market_state);

            // pending write because best market price did not change
            assert_eq!(remover.pending_write, true);

            // No change in opposite side price
            assert_eq!(market_state.best_bid_price, order_id_bid.price_in_ticks);

            // Best price not changed
            assert_eq!(market_state.best_ask_price, order_id_0.price_in_ticks);

            // No change in outer index
            assert_eq!(remover.outer_index_unchecked(), outer_index_0);

            // 4. Write list
            remover.write_prepared_indices(&mut ctx, &mut market_state);
            // No change because group is still active
            assert_eq!(market_state.asks_outer_indices, 1);

            // Check bitmap group
            let mut expected_bitmap_group = BitmapGroup::default();
            expected_bitmap_group.inner[0] = 0b00000001;
            expected_bitmap_group.inner[2] = 0b00000001;
            assert_eq!(
                remover.group_position_remover.bitmap_group,
                expected_bitmap_group
            );

            let read_bitmap_group = BitmapGroup::new_from_slot(&ctx, outer_index_0);
            assert_eq!(read_bitmap_group, expected_bitmap_group);
        }

        #[test]
        fn test_search_one_but_remove_another_in_same_outer_index() {
            let mut ctx = ArbContext::new();
            let side = Side::Ask;

            let outer_index_count = 1;
            let outer_index_0 = OuterIndex::new(1);

            // Write outer indices to slot
            let list_key = ListKey { index: 0, side };
            let mut list_slot = ListSlot::default();
            list_slot.set(0, outer_index_0);
            list_slot.write_to_slot(&mut ctx, &list_key);

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
            enable_order_id(&mut ctx, order_id_bid);
            enable_order_id(&mut ctx, order_id_0);
            enable_order_id(&mut ctx, order_id_1);

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
            let found_0 =
                remover.order_id_is_active(&mut ctx, &market_state.get_prices(), order_id_0);
            assert!(found_0);
            assert_eq!(remover.order_id(), order_id_0);

            // 2. Search order_id_1
            let found_1 =
                remover.order_id_is_active(&mut ctx, &market_state.get_prices(), order_id_1);
            assert!(found_1);
            assert_eq!(remover.order_id(), order_id_1);
            assert_eq!(remover.outer_index_unchecked(), outer_index_0);
            assert_eq!(
                remover.outer_index_remover.cached_outer_index.unwrap(),
                outer_index_0
            );
            assert_eq!(remover.outer_index_remover.cache, vec![]);

            // 3. Remove order_id_1
            remover.remove_order(&mut ctx, &mut market_state);

            // Pending write because best price did not change
            // TODO fix
            assert_eq!(remover.pending_write, true);

            // No change in opposite side price
            assert_eq!(market_state.best_bid_price, order_id_bid.price_in_ticks);

            // Best price not changed
            assert_eq!(market_state.best_ask_price, order_id_0.price_in_ticks);

            // No change in outer index
            assert_eq!(remover.outer_index_unchecked(), outer_index_0);

            // 4. Write list
            remover.write_prepared_indices(&mut ctx, &mut market_state);
            // No change because group is still active
            assert_eq!(market_state.asks_outer_indices, 1);

            // Check bitmap group
            let mut expected_bitmap_group = BitmapGroup::default();
            expected_bitmap_group.inner[0] = 0b00000001;
            expected_bitmap_group.inner[2] = 0b00000001;
            assert_eq!(
                remover.group_position_remover.bitmap_group,
                expected_bitmap_group
            );

            let read_bitmap_group = BitmapGroup::new_from_slot(&ctx, outer_index_0);
            assert_eq!(read_bitmap_group, expected_bitmap_group);
        }

        #[test]
        fn test_search_one_but_remove_another_in_different_outer_index() {
            let mut ctx = ArbContext::new();
            let side = Side::Ask;

            let outer_index_count = 2;
            let outer_index_0 = OuterIndex::new(1);
            let outer_index_1 = OuterIndex::new(2);

            // Write outer indices to slot
            let list_key = ListKey { index: 0, side };
            let mut list_slot = ListSlot::default();
            list_slot.set(0, outer_index_1);
            list_slot.set(1, outer_index_0); // smaller index is at end of the list
            list_slot.write_to_slot(&mut ctx, &list_key);

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
            enable_order_id(&mut ctx, order_id_bid);
            enable_order_id(&mut ctx, order_id_0);
            enable_order_id(&mut ctx, order_id_1);

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
            let found_0 =
                remover.order_id_is_active(&mut ctx, &market_state.get_prices(), order_id_0);
            assert!(found_0);
            assert_eq!(remover.order_id(), order_id_0);

            // 2. Search order_id_1
            let found_1 =
                remover.order_id_is_active(&mut ctx, &market_state.get_prices(), order_id_1);

            // TODO fix here. Ghost values should only be cleared in the outermost group
            assert!(found_1);

            assert_eq!(remover.order_id(), order_id_1);
            assert_eq!(remover.outer_index_unchecked(), outer_index_1);
            assert_eq!(
                remover.outer_index_remover.cached_outer_index.unwrap(),
                outer_index_1
            );
            assert_eq!(remover.outer_index_remover.cache, vec![outer_index_0]);

            // 3. Remove
            remover.remove_order(&mut ctx, &mut market_state);

            // Pending write is false because group was closed and because this is not the
            // outermost group
            assert_eq!(remover.pending_write, false);

            // No change in opposite side price
            assert_eq!(market_state.best_bid_price, order_id_bid.price_in_ticks);

            // Best price not changed
            assert_eq!(market_state.best_ask_price, order_id_0.price_in_ticks);

            // Outer index cleared because the group closed
            assert!(remover.last_outer_index().is_none());

            // 4. Write list
            remover.write_prepared_indices(&mut ctx, &mut market_state);
            // Changed because outer_index_0 was closed
            assert_eq!(market_state.asks_outer_indices, 1);

            // Check bitmap group
            let mut expected_bitmap_group_0 = BitmapGroup::default();
            expected_bitmap_group_0.inner[0] = 0b00000001;
            expected_bitmap_group_0.inner[2] = 0b00000001;

            let read_bitmap_group_0 = BitmapGroup::new_from_slot(&ctx, outer_index_0);
            assert_eq!(read_bitmap_group_0, expected_bitmap_group_0);

            let expected_cached_bitmap_group_1 = BitmapGroup::default();
            assert_eq!(
                remover.group_position_remover.bitmap_group,
                expected_cached_bitmap_group_1
            );

            // bitmap group 1 is not written to slot. Slot will give the previous value
            let mut expected_written_bitmap_group_1 = BitmapGroup::default();
            expected_written_bitmap_group_1.inner[0] = 0b00000001;
            let read_bitmap_group_1 = BitmapGroup::new_from_slot(&ctx, outer_index_1);
            assert_eq!(read_bitmap_group_1, expected_written_bitmap_group_1);
        }
    }

    mod traverse_consecutive_orders {
        use super::*;

        #[test]
        fn traverse_some_asks_on_inner_index() {
            let mut ctx = &mut ArbContext::new();
            let side = Side::Ask;

            let outer_index_count = 2;
            let outer_index_0 = OuterIndex::new(1);
            let outer_index_1 = OuterIndex::new(2);

            let list_key = ListKey { index: 0, side };
            let mut list_slot = ListSlot::default();
            list_slot.set(0, outer_index_1);
            list_slot.set(1, outer_index_0);
            list_slot.write_to_slot(ctx, &list_key);

            let best_market_price = Ticks::from_indices(outer_index_0, InnerIndex::new(1));
            let best_opposite_price = Ticks::from_indices(OuterIndex::ZERO, InnerIndex::ONE);

            let mut market_state = MarketState {
                collected_quote_lot_fees: QuoteLots::ZERO,
                unclaimed_quote_lot_fees: QuoteLots::ZERO,
                bids_outer_indices: 1,
                asks_outer_indices: outer_index_count,
                best_bid_price: best_opposite_price,
                best_ask_price: best_market_price,
            };

            // Garbage bit- doesn't belong to asks
            let order_id_garbage = OrderId {
                price_in_ticks: Ticks::from_indices(outer_index_0, InnerIndex::ZERO),
                resting_order_index: RestingOrderIndex::ZERO,
            };

            // Order at the best price
            let order_id_0 = OrderId {
                price_in_ticks: best_market_price,
                resting_order_index: RestingOrderIndex::ZERO,
            };
            // Order on same inner index
            let order_id_1 = OrderId {
                price_in_ticks: best_market_price,
                resting_order_index: RestingOrderIndex::new(2),
            };

            // Order on same outer index
            let order_id_2 = OrderId {
                price_in_ticks: Ticks::from_indices(outer_index_0, InnerIndex::new(2)),
                resting_order_index: RestingOrderIndex::ZERO,
            };

            // Order on different outer index
            let order_id_3 = OrderId {
                price_in_ticks: Ticks::from_indices(outer_index_1, InnerIndex::ZERO),
                resting_order_index: RestingOrderIndex::ZERO,
            };

            enable_order_id(ctx, order_id_garbage);
            enable_order_id(ctx, order_id_0);
            enable_order_id(ctx, order_id_1);
            enable_order_id(ctx, order_id_2);
            enable_order_id(ctx, order_id_3);

            let mut remover = OrderIdRemover::new_for_matching(
                &mut ctx,
                side,
                outer_index_count,
                &market_state.get_prices(),
            )
            .unwrap();

            // order id 0 is read. Garbage bit is skipped
            assert_eq!(remover.best_active_order_id(ctx).unwrap(), order_id_0);

            // Calling best_active_order_id() without clearing gives same result
            assert_eq!(remover.best_active_order_id(ctx).unwrap(), order_id_0);

            // TODO check if garbage bit was cleared
            let mut expected_bitmap_group_after_read = BitmapGroup::default();
            // garbage bit at inner index 0 cleared
            expected_bitmap_group_after_read.inner[1] = 0b0000_0101;
            expected_bitmap_group_after_read.inner[2] = 0b0000_0001;
            assert_eq!(
                remover.group_position_remover.bitmap_group,
                expected_bitmap_group_after_read
            );

            remover.remove_order(ctx, &mut market_state); // remove 0
            assert_eq!(remover.best_active_order_id(ctx).unwrap(), order_id_1);
            assert_eq!(market_state.best_price(side), order_id_1.price_in_ticks);

            remover.write_prepared_indices(ctx, &mut market_state);
            assert_eq!(market_state.asks_outer_indices, 2);

            let read_list_slot = ListSlot::new_from_slot(ctx, list_key);
            assert_eq!(read_list_slot, list_slot);

            // Updated bitmap group written. The outermost bit and ghost bit was cleared
            // Error- garbage bit not cleared
            let mut expected_bitmap_group_0 = BitmapGroup::default();
            // garbage bit at inner index 0 cleared
            expected_bitmap_group_0.inner[1] = 0b0000_0100;
            expected_bitmap_group_0.inner[2] = 0b0000_0001;
            assert_eq!(
                BitmapGroup::new_from_slot(ctx, outer_index_0),
                expected_bitmap_group_0
            );
        }

        #[test]
        fn traverse_some_asks_on_outer_index() {
            let mut ctx = &mut ArbContext::new();
            let side = Side::Ask;

            let outer_index_count = 2;
            let outer_index_0 = OuterIndex::new(1);
            let outer_index_1 = OuterIndex::new(2);

            let list_key = ListKey { index: 0, side };
            let mut list_slot = ListSlot::default();
            list_slot.set(0, outer_index_1);
            list_slot.set(1, outer_index_0);
            list_slot.write_to_slot(ctx, &list_key);

            let best_market_price = Ticks::from_indices(outer_index_0, InnerIndex::new(1));
            let best_opposite_price = Ticks::from_indices(OuterIndex::ZERO, InnerIndex::ONE);

            let mut market_state = MarketState {
                collected_quote_lot_fees: QuoteLots::ZERO,
                unclaimed_quote_lot_fees: QuoteLots::ZERO,
                bids_outer_indices: 1,
                asks_outer_indices: outer_index_count,
                best_bid_price: best_opposite_price,
                best_ask_price: best_market_price,
            };

            // Garbage bit- doesn't belong to asks
            let order_id_garbage = OrderId {
                price_in_ticks: Ticks::from_indices(outer_index_0, InnerIndex::ZERO),
                resting_order_index: RestingOrderIndex::ZERO,
            };

            // Order at the best price
            let order_id_0 = OrderId {
                price_in_ticks: best_market_price,
                resting_order_index: RestingOrderIndex::ZERO,
            };
            // Order on same inner index
            let order_id_1 = OrderId {
                price_in_ticks: best_market_price,
                resting_order_index: RestingOrderIndex::new(2),
            };

            // Order on same outer index
            let order_id_2 = OrderId {
                price_in_ticks: Ticks::from_indices(outer_index_0, InnerIndex::new(2)),
                resting_order_index: RestingOrderIndex::ZERO,
            };

            // Order on different outer index
            let order_id_3 = OrderId {
                price_in_ticks: Ticks::from_indices(outer_index_1, InnerIndex::ZERO),
                resting_order_index: RestingOrderIndex::ZERO,
            };

            enable_order_id(ctx, order_id_garbage);
            enable_order_id(ctx, order_id_0);
            enable_order_id(ctx, order_id_1);
            enable_order_id(ctx, order_id_2);
            enable_order_id(ctx, order_id_3);

            let mut remover = OrderIdRemover::new_for_matching(
                &mut ctx,
                side,
                outer_index_count,
                &market_state.get_prices(),
            )
            .unwrap();

            // order id 0 is read. Garbage bit is skipped
            assert_eq!(remover.best_active_order_id(ctx).unwrap(), order_id_0);

            remover.remove_order(ctx, &mut market_state); // remove 0
            assert_eq!(remover.best_active_order_id(ctx).unwrap(), order_id_1);
            assert_eq!(market_state.best_price(side), order_id_1.price_in_ticks);

            remover.remove_order(ctx, &mut market_state); // remove 1
            assert_eq!(remover.best_active_order_id(ctx).unwrap(), order_id_2);
            assert_eq!(market_state.best_price(side), order_id_2.price_in_ticks);

            remover.write_prepared_indices(ctx, &mut market_state);
            assert_eq!(market_state.asks_outer_indices, 2);

            let read_list_slot = ListSlot::new_from_slot(ctx, list_key);
            assert_eq!(read_list_slot, list_slot);

            // Bitmap group not written to slot since market price was decremented
            let mut expected_bitmap_group_0 = BitmapGroup::default();
            expected_bitmap_group_0.inner[0] = 0b0000_0001;
            expected_bitmap_group_0.inner[1] = 0b0000_0101;
            expected_bitmap_group_0.inner[2] = 0b0000_0001;
            assert_eq!(
                BitmapGroup::new_from_slot(ctx, outer_index_0),
                expected_bitmap_group_0
            );
        }

        #[test]
        fn traverse_all_asks_on_outer_index() {
            let mut ctx = &mut ArbContext::new();
            let side = Side::Ask;

            let outer_index_count = 2;
            let outer_index_0 = OuterIndex::new(1);
            let outer_index_1 = OuterIndex::new(2);

            let list_key = ListKey { index: 0, side };
            let mut list_slot = ListSlot::default();
            list_slot.set(0, outer_index_1);
            list_slot.set(1, outer_index_0);
            list_slot.write_to_slot(ctx, &list_key);

            let best_market_price = Ticks::from_indices(outer_index_0, InnerIndex::new(1));
            let best_opposite_price = Ticks::from_indices(OuterIndex::ZERO, InnerIndex::ONE);

            let mut market_state = MarketState {
                collected_quote_lot_fees: QuoteLots::ZERO,
                unclaimed_quote_lot_fees: QuoteLots::ZERO,
                bids_outer_indices: 1,
                asks_outer_indices: outer_index_count,
                best_bid_price: best_opposite_price,
                best_ask_price: best_market_price,
            };

            // Garbage bit- doesn't belong to asks
            let order_id_garbage = OrderId {
                price_in_ticks: Ticks::from_indices(outer_index_0, InnerIndex::ZERO),
                resting_order_index: RestingOrderIndex::ZERO,
            };

            // Order at the best price
            let order_id_0 = OrderId {
                price_in_ticks: best_market_price,
                resting_order_index: RestingOrderIndex::ZERO,
            };
            // Order on same inner index
            let order_id_1 = OrderId {
                price_in_ticks: best_market_price,
                resting_order_index: RestingOrderIndex::new(2),
            };

            // Order on same outer index
            let order_id_2 = OrderId {
                price_in_ticks: Ticks::from_indices(outer_index_0, InnerIndex::new(2)),
                resting_order_index: RestingOrderIndex::ZERO,
            };

            // Order on different outer index
            let order_id_3 = OrderId {
                price_in_ticks: Ticks::from_indices(outer_index_1, InnerIndex::ZERO),
                resting_order_index: RestingOrderIndex::ZERO,
            };

            enable_order_id(ctx, order_id_garbage);
            enable_order_id(ctx, order_id_0);
            enable_order_id(ctx, order_id_1);
            enable_order_id(ctx, order_id_2);
            enable_order_id(ctx, order_id_3);

            let mut remover = OrderIdRemover::new_for_matching(
                &mut ctx,
                side,
                outer_index_count,
                &market_state.get_prices(),
            )
            .unwrap();

            // order id 0 is read. Garbage bit is skipped
            assert_eq!(remover.best_active_order_id(ctx).unwrap(), order_id_0);

            remover.remove_order(ctx, &mut market_state); // remove 0
            assert_eq!(remover.best_active_order_id(ctx).unwrap(), order_id_1);
            assert_eq!(market_state.best_price(side), order_id_1.price_in_ticks);

            remover.remove_order(ctx, &mut market_state); // remove 1
            assert_eq!(remover.best_active_order_id(ctx).unwrap(), order_id_2);
            assert_eq!(market_state.best_price(side), order_id_2.price_in_ticks);

            remover.remove_order(ctx, &mut market_state); // remove 2
            assert_eq!(remover.best_active_order_id(ctx).unwrap(), order_id_3);
            assert_eq!(market_state.best_price(side), order_id_3.price_in_ticks);

            remover.write_prepared_indices(ctx, &mut market_state);

            // Since ask outer index count was decremented, no need to write updated
            // list slot
            assert_eq!(market_state.asks_outer_indices, 1);
            let read_list_slot = ListSlot::new_from_slot(ctx, list_key);
            assert_eq!(read_list_slot, list_slot);

            // Bitmap group not written to slot since outer index was removed
            let mut expected_bitmap_group_0 = BitmapGroup::default();
            expected_bitmap_group_0.inner[0] = 0b0000_0001;
            expected_bitmap_group_0.inner[1] = 0b0000_0101;
            expected_bitmap_group_0.inner[2] = 0b0000_0001;
            assert_eq!(
                BitmapGroup::new_from_slot(ctx, outer_index_0),
                expected_bitmap_group_0
            );
        }

        #[test]
        fn traverse_asks_across_outer_indices() {
            let mut ctx = &mut ArbContext::new();
            let side = Side::Ask;

            let outer_index_count = 2;
            let outer_index_0 = OuterIndex::new(1);
            let outer_index_1 = OuterIndex::new(2);

            let list_key = ListKey { index: 0, side };
            let mut list_slot = ListSlot::default();
            list_slot.set(0, outer_index_1);
            list_slot.set(1, outer_index_0);
            list_slot.write_to_slot(ctx, &list_key);

            let best_market_price = Ticks::from_indices(outer_index_0, InnerIndex::new(1));
            let best_opposite_price = Ticks::from_indices(OuterIndex::ZERO, InnerIndex::ONE);

            let mut market_state = MarketState {
                collected_quote_lot_fees: QuoteLots::ZERO,
                unclaimed_quote_lot_fees: QuoteLots::ZERO,
                bids_outer_indices: 1,
                asks_outer_indices: outer_index_count,
                best_bid_price: best_opposite_price,
                best_ask_price: best_market_price,
            };

            // Garbage bit- doesn't belong to asks
            let order_id_garbage = OrderId {
                price_in_ticks: Ticks::from_indices(outer_index_0, InnerIndex::ZERO),
                resting_order_index: RestingOrderIndex::ZERO,
            };

            // Order at the best price
            let order_id_0 = OrderId {
                price_in_ticks: best_market_price,
                resting_order_index: RestingOrderIndex::ZERO,
            };
            // Order on same inner index
            let order_id_1 = OrderId {
                price_in_ticks: best_market_price,
                resting_order_index: RestingOrderIndex::new(2),
            };

            // Order on same outer index
            let order_id_2 = OrderId {
                price_in_ticks: Ticks::from_indices(outer_index_0, InnerIndex::new(2)),
                resting_order_index: RestingOrderIndex::ZERO,
            };

            // Order on different outer index
            let order_id_3 = OrderId {
                price_in_ticks: Ticks::from_indices(outer_index_1, InnerIndex::ZERO),
                resting_order_index: RestingOrderIndex::ZERO,
            };

            enable_order_id(ctx, order_id_garbage);
            enable_order_id(ctx, order_id_0);
            enable_order_id(ctx, order_id_1);
            enable_order_id(ctx, order_id_2);
            enable_order_id(ctx, order_id_3);

            let mut remover = OrderIdRemover::new_for_matching(
                &mut ctx,
                side,
                outer_index_count,
                &market_state.get_prices(),
            )
            .unwrap();

            // order id 0 is read. Garbage bit is skipped
            assert_eq!(remover.best_active_order_id(ctx).unwrap(), order_id_0);

            remover.remove_order(ctx, &mut market_state); // remove 0
            assert_eq!(remover.best_active_order_id(ctx).unwrap(), order_id_1);
            assert_eq!(market_state.best_price(side), order_id_1.price_in_ticks);

            remover.remove_order(ctx, &mut market_state); // remove 1
            assert_eq!(remover.best_active_order_id(ctx).unwrap(), order_id_2);
            assert_eq!(market_state.best_price(side), order_id_2.price_in_ticks);

            remover.remove_order(ctx, &mut market_state); // remove 2
            assert_eq!(remover.best_active_order_id(ctx).unwrap(), order_id_3);
            assert_eq!(market_state.best_price(side), order_id_3.price_in_ticks);

            remover.remove_order(ctx, &mut market_state); // remove 3
                                                          // All order ids exhausted
            assert!(remover.best_active_order_id(ctx).is_none());
            assert_eq!(market_state.best_price(side), Ticks::MAX);

            // TODO problem here
            remover.write_prepared_indices(ctx, &mut market_state);

            // Since ask outer index count was decremented, no need to write updated
            // list slot
            assert_eq!(market_state.asks_outer_indices, 0);
            let read_list_slot = ListSlot::new_from_slot(ctx, list_key);
            assert_eq!(read_list_slot, list_slot);

            // Bitmap group not written to slot since outer index was removed
            let mut expected_bitmap_group_0 = BitmapGroup::default();
            expected_bitmap_group_0.inner[0] = 0b0000_0001;
            expected_bitmap_group_0.inner[1] = 0b0000_0101;
            expected_bitmap_group_0.inner[2] = 0b0000_0001;
            assert_eq!(
                BitmapGroup::new_from_slot(ctx, outer_index_0),
                expected_bitmap_group_0
            );

            let mut expected_bitmap_group_1 = BitmapGroup::default();
            expected_bitmap_group_1.inner[0] = 0b0000_0001;
            assert_eq!(
                BitmapGroup::new_from_slot(ctx, outer_index_1),
                expected_bitmap_group_1
            );
        }
    }
}
