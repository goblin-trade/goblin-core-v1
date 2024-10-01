use crate::{
    quantities::Ticks,
    state::{
        bitmap_group::BitmapGroup,
        order::{group_position::GroupPosition, order_id::OrderId},
        ArbContext, InnerIndex, MarketPrices, OuterIndex, RestingOrderIndex, Side,
    },
};

/// Facilitates efficient batch deactivations at GroupPositions
pub struct GroupPositionRemover {
    // TODO use ActiveGroupPositionIterator as inner
    /// Whether for bids or asks
    /// Traverse upwards (ascending) for asks and downwards (descending) for bids
    pub side: Side,

    /// The current bitmap group pending a write. This allows us to perform multiple
    /// updates in a bitmap group with a single slot load. This value is written to slot
    /// when a new outer index is encountered.
    pub bitmap_group: BitmapGroup,

    /// Outer index corresponding to `bitmap_group`
    /// TODO remove, pass as params
    pub last_outer_index: Option<OuterIndex>,

    /// The current group position used to paginate and for deactivate bits.
    /// The bit at group_position can either be active or inactive.
    pub group_position: GroupPosition,

    /// Whether the bitmap group was updated in memory and is pending a write.
    /// write_last_bitmap_group() should write to slot only if this is true.
    pub pending_write: bool,
}

impl GroupPositionRemover {
    pub fn new(side: Side) -> Self {
        GroupPositionRemover {
            side,
            bitmap_group: BitmapGroup::default(),
            last_outer_index: None,
            // Default group position starts at the starting position of a given side
            group_position: GroupPosition::initial_for_side(side),
            pending_write: false,
        }
    }

    /// The last searched order ID
    pub fn last_searched_order_id(&self) -> Option<OrderId> {
        self.last_outer_index
            .map(|outer_index| OrderId::from_group_position(self.group_position, outer_index))
    }

    /// Paginates to the given position and tells whether the bit is active
    ///
    /// Externally ensure that load_outer_index() was called first otherwise
    /// this will give a dummy value
    ///
    pub fn is_position_active(&mut self, group_position: GroupPosition) -> bool {
        self.group_position = group_position;
        self.bitmap_group.is_position_active(group_position)
    }

    pub fn is_inner_index_active(&self, inner_index: InnerIndex) -> bool {
        self.bitmap_group.is_inner_index_active(inner_index)
    }

    /// Deactivates `self.group_position` and conditionally enables or disables `pending_write`
    ///
    /// Sets pending_write to false if market price updates or if the whole group is cleared,
    /// else sets it to true.
    ///
    /// # Arguments
    ///
    /// * `best_market_price` - Best market price for the current side used to
    /// conditionally update pending_write state
    ///
    pub fn deactivate_current_group_position(&mut self, best_market_price: Ticks) {
        if let Some(outer_index) = self.last_outer_index {
            let GroupPosition {
                inner_index,
                resting_order_index,
            } = self.group_position;
            let current_price = Ticks::from_indices(outer_index, inner_index);

            let mut bitmap = self.bitmap_group.get_bitmap_mut(&inner_index);
            bitmap.clear(&resting_order_index);

            // Do not write if
            // - the best price updated, i.e. the outermost bitmap closed. In this case
            // we will update the best price in market state.
            // - the entire group was deactivated. In this case we will remove the outer
            // index in the list.
            self.pending_write = !(current_price == best_market_price && bitmap.is_empty()
                || self.bitmap_group.is_inactive(self.side, None));
        }
    }

    /// Get price of the best active order in the current bitmap group
    /// from a given position (inclusive) and onwards.
    ///
    /// Used to find the best market price after removing bits from the group.
    /// This does NOT update `self.group_position` because removal can happen
    /// deeper in the group whereas we need the best market price.
    ///
    /// # Arguments
    ///
    /// * `starting_index` - Search beginning from this index (inclusive) if Some,
    /// else begin lookup from the edge of the bitmap group.
    ///
    pub fn get_best_price_in_current(
        &self,
        starting_index_inclusive: Option<InnerIndex>,
    ) -> Option<Ticks> {
        if let (Some(outer_index), Some(inner_index)) = (
            self.last_outer_index,
            self.bitmap_group
                .best_active_inner_index(self.side, starting_index_inclusive),
        ) {
            Some(Ticks::from_indices(outer_index, inner_index))
        } else {
            None
        }
    }

    /// Clear garbage bits in the bitmap group that fall between best market prices
    ///
    /// Externally ensure this is not called if outer index is not loaded
    ///
    /// # Arguments
    ///
    /// * `best_market_prices`
    ///
    pub fn clear_garbage_bits(&mut self, best_market_prices: &MarketPrices) {
        debug_assert!(self.last_outer_index.is_some());

        let outer_index = unsafe { self.last_outer_index.unwrap_unchecked() };
        self.bitmap_group
            .clear_garbage_bits(outer_index, best_market_prices);
    }

    /// Try traversing to the next active group position in the current bitmap group.
    /// If an active position is present, updates `group_position` and returns true.
    /// Else returns false.
    pub fn try_traverse_to_best_active_position(&mut self) -> bool {
        let next_active_group_position = self
            .bitmap_group
            .best_active_group_position(self.side, self.group_position);

        if let Some(group_position) = next_active_group_position {
            self.group_position = group_position;

            return true;
        }

        false
    }

    /// Whether the bitmap group has been inactivated for `self.side`. It accounts for
    /// and excludes bits belonging to the opposite side during lookup.
    ///
    /// Externally ensure that `last_outer_index` is not None and has active bits for `side`, so there is
    /// no overflow or underflow when we add or subtract from `best_opposite_inner_index`.
    ///
    /// # Arguments
    ///
    /// * `best_opposite_price`
    ///
    pub fn is_group_inactive(&self, best_opposite_price: Ticks) -> bool {
        let start_index = if self.last_outer_index == Some(best_opposite_price.outer_index()) {
            // Overflow or underflow would happen only if the most extreme bitmap is occupied
            // by opposite side bits. This is not possible because active bits for `side`
            // are guaranteed to be present.

            let best_opposite_inner_index = best_opposite_price.inner_index();
            Some(if self.side == Side::Bid {
                best_opposite_inner_index - InnerIndex::ONE
            } else {
                best_opposite_inner_index + InnerIndex::ONE
            })
        } else {
            None
        };

        self.bitmap_group.is_inactive(self.side, start_index)
    }

    /// Loads a new bitmap group for the new outer index. The previous group is flushed.
    /// No-op if outer index does not change
    ///
    /// # Externally ensure that
    ///
    /// * we always move away from the centre
    /// * outer_index is active and non-empty
    ///
    pub fn load_outer_index(&mut self, ctx: &mut ArbContext, outer_index: OuterIndex) {
        if self.last_outer_index == Some(outer_index) {
            return;
        }
        // Outer index changed. Flush the old bitmap group to slot.
        self.flush_bitmap_group(ctx);

        self.last_outer_index = Some(outer_index);

        // self.last_searched_group_position = None;
        self.group_position = GroupPosition::initial_for_side(self.side);

        self.bitmap_group = BitmapGroup::new_from_slot(ctx, outer_index);
    }

    /// Flush the cached bitmap group to slot
    ///
    /// This should be called before moving to a new outer index
    ///
    /// # Arguments
    ///
    /// * `ctx`
    ///
    pub fn flush_bitmap_group(&mut self, ctx: &mut ArbContext) {
        if !self.pending_write {
            return;
        }

        if let Some(last_index) = self.last_outer_index {
            self.bitmap_group.write_to_slot(ctx, &last_index);
            self.pending_write = false;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::state::RestingOrderIndex;

    use super::*;

    fn load_and_deactivate(
        remover: &mut GroupPositionRemover,
        group_position: GroupPosition,
        best_market_price: Ticks,
    ) {
        let present = remover.is_position_active(group_position);
        assert_eq!(present, true);

        remover.deactivate_current_group_position(best_market_price);
        let present_after_deactivation = remover.bitmap_group.is_position_active(group_position);
        assert_eq!(present_after_deactivation, false);
    }

    // Test cases where cleared bitmap group is not written (pending_write is false)
    // - Outermost tick closed
    // - Whole group closed
    //
    // Cases where pending_write is true
    // - Remove from outermost but last price does not change
    // - Remove behind outermost

    #[test]
    pub fn test_pending_write_behavior_on_sequential_removals_ask() {
        let side = Side::Ask;
        let outer_index = OuterIndex::new(1);

        // Write initial data to slot
        let mut bitmap_group = BitmapGroup::default();
        // Best opposite price
        bitmap_group.inner[0] = 0b00000001;

        // Best market price- two resting orders
        bitmap_group.inner[1] = 0b10000001;

        // Next inner price. Rest of the group is empty
        bitmap_group.inner[2] = 0b00000001;

        let mut remover = GroupPositionRemover {
            side,
            bitmap_group,
            last_outer_index: Some(outer_index),
            group_position: GroupPosition::initial_for_side(side),
            pending_write: false,
        };

        let mut best_market_price = Ticks::from_indices(outer_index, InnerIndex::ONE);

        // 1. Remove first bit on outermost tick.
        // - Since the tick remains active `pending_write` is true.
        // - Best price does not update
        load_and_deactivate(
            &mut remover,
            GroupPosition {
                inner_index: InnerIndex::ONE,
                resting_order_index: RestingOrderIndex::ZERO,
            },
            best_market_price,
        );
        assert_eq!(remover.pending_write, true);

        // No change in market price
        best_market_price = remover
            .get_best_price_in_current(Some(InnerIndex::ONE))
            .unwrap();
        assert_eq!(
            best_market_price,
            Ticks::from_indices(outer_index, InnerIndex::ONE)
        );

        // 2. Remove final bit on the outermost tick
        // - The outermost tick is closed so `pending_write` is false
        // - Best price updated
        load_and_deactivate(
            &mut remover,
            GroupPosition {
                inner_index: InnerIndex::ONE,
                resting_order_index: RestingOrderIndex::new(7),
            },
            best_market_price,
        );
        assert_eq!(remover.pending_write, false);

        // Market price changed
        best_market_price = remover
            .get_best_price_in_current(Some(best_market_price.inner_index()))
            .unwrap();
        assert_eq!(
            best_market_price,
            Ticks::from_indices(outer_index, InnerIndex::new(2))
        );

        // 3. Remove final bit in the group
        // - The whole group is closed so `pending_write` is false
        // - Best price updated
        load_and_deactivate(
            &mut remover,
            GroupPosition {
                inner_index: InnerIndex::new(2),
                resting_order_index: RestingOrderIndex::new(0),
            },
            best_market_price,
        );
        assert_eq!(remover.pending_write, false);

        // Best price is None because the group is cleared
        assert_eq!(
            remover.get_best_price_in_current(Some(best_market_price.inner_index())),
            None
        );
    }

    #[test]
    pub fn test_pending_write_behavior_on_removing_behind_best_price_ask() {
        let side = Side::Ask;
        let outer_index = OuterIndex::new(1);

        // Write initial data to slot
        let mut bitmap_group = BitmapGroup::default();
        // Best opposite price
        bitmap_group.inner[0] = 0b00000001;

        // Best market price- two resting orders
        bitmap_group.inner[1] = 0b10000001;

        // Next inner price. Rest of the group is empty
        bitmap_group.inner[2] = 0b00000001;

        let mut remover = GroupPositionRemover {
            side,
            bitmap_group,
            last_outer_index: Some(outer_index),
            group_position: GroupPosition::initial_for_side(side),
            pending_write: false,
        };

        let best_market_price = Ticks::from_indices(outer_index, InnerIndex::ONE);

        // 3. Remove the bit at inner index 2
        // - Best price is unchanged because ticks at best tick are not removed
        // - `pending_write` is true
        load_and_deactivate(
            &mut remover,
            GroupPosition {
                inner_index: InnerIndex::new(2),
                resting_order_index: RestingOrderIndex::new(0),
            },
            best_market_price,
        );
        assert_eq!(remover.pending_write, true);

        // No change in best price
        let new_best_market_price = remover
            .get_best_price_in_current(Some(best_market_price.inner_index()))
            .unwrap();
        assert_eq!(new_best_market_price, best_market_price,);
    }

    #[test]
    pub fn test_remove_in_non_outermost_group_ask() {
        let side = Side::Ask;
        let outer_index = OuterIndex::new(1);

        let best_market_price = Ticks::from_indices(OuterIndex::new(2), InnerIndex::new(0));

        // Write initial data to slot
        let mut bitmap_group = BitmapGroup::default();
        bitmap_group.inner[1] = 0b10000001;
        bitmap_group.inner[2] = 0b00000001;

        let mut remover = GroupPositionRemover {
            side,
            bitmap_group,
            last_outer_index: Some(outer_index),
            group_position: GroupPosition::initial_for_side(side),
            pending_write: false,
        };

        // 1. Remove first bit on outermost
        // - `pending_write` is true because group did not clear. The best price change
        // condition does not apply on non-outermost groups
        load_and_deactivate(
            &mut remover,
            GroupPosition {
                inner_index: InnerIndex::new(1),
                resting_order_index: RestingOrderIndex::new(0),
            },
            best_market_price,
        );
        assert_eq!(remover.pending_write, true);

        // 2. Remove final bit on outermost
        // - `pending_write` is true because group did not clear
        load_and_deactivate(
            &mut remover,
            GroupPosition {
                inner_index: InnerIndex::new(1),
                resting_order_index: RestingOrderIndex::new(7),
            },
            best_market_price,
        );
        assert_eq!(remover.pending_write, true);

        // 3. Close the last bit in group
        // - `pending_write` is false because the whole group cleared
        load_and_deactivate(
            &mut remover,
            GroupPosition {
                inner_index: InnerIndex::new(2),
                resting_order_index: RestingOrderIndex::new(0),
            },
            best_market_price,
        );
        assert_eq!(remover.pending_write, false);
    }

    #[test]
    pub fn test_pending_write_behavior_on_sequential_removals_bid() {
        let side = Side::Bid;
        let outer_index = OuterIndex::new(1);

        // Write initial data to slot
        let mut bitmap_group = BitmapGroup::default();
        // Best opposite price
        bitmap_group.inner[31] = 0b00000001;

        // Best market price - two resting orders
        bitmap_group.inner[30] = 0b10000001;

        // Next inner price. Rest of the group is empty
        bitmap_group.inner[29] = 0b00000001;

        let mut remover = GroupPositionRemover {
            side,
            bitmap_group,
            last_outer_index: Some(outer_index),
            group_position: GroupPosition::initial_for_side(side),
            pending_write: false,
        };

        let mut best_market_price = Ticks::from_indices(outer_index, InnerIndex::new(30));

        // 1. Remove first bit on outermost tick.
        load_and_deactivate(
            &mut remover,
            GroupPosition {
                inner_index: InnerIndex::new(30),
                resting_order_index: RestingOrderIndex::ZERO,
            },
            best_market_price,
        );
        assert_eq!(remover.pending_write, true);

        // No change in market price
        best_market_price = remover
            .get_best_price_in_current(Some(InnerIndex::new(30)))
            .unwrap();
        assert_eq!(
            best_market_price,
            Ticks::from_indices(outer_index, InnerIndex::new(30))
        );

        // 2. Remove final bit on the outermost tick
        load_and_deactivate(
            &mut remover,
            GroupPosition {
                inner_index: InnerIndex::new(30),
                resting_order_index: RestingOrderIndex::new(7),
            },
            best_market_price,
        );
        assert_eq!(remover.pending_write, false);

        // Market price changed
        best_market_price = remover
            .get_best_price_in_current(Some(best_market_price.inner_index()))
            .unwrap();
        assert_eq!(
            best_market_price,
            Ticks::from_indices(outer_index, InnerIndex::new(29))
        );

        // 3. Remove final bit in the group
        load_and_deactivate(
            &mut remover,
            GroupPosition {
                inner_index: InnerIndex::new(29),
                resting_order_index: RestingOrderIndex::new(0),
            },
            best_market_price,
        );
        assert_eq!(remover.pending_write, false);

        // Best price is None because the group is cleared
        assert_eq!(
            remover.get_best_price_in_current(Some(best_market_price.inner_index())),
            None
        );
    }

    #[test]
    pub fn test_pending_write_behavior_on_removing_behind_best_price_bid() {
        let side = Side::Bid;
        let outer_index = OuterIndex::new(1);

        // Write initial data to slot
        let mut bitmap_group = BitmapGroup::default();
        // Best opposite price
        bitmap_group.inner[31] = 0b00000001;

        // Best market price - two resting orders
        bitmap_group.inner[30] = 0b10000001;

        // Next inner price. Rest of the group is empty
        bitmap_group.inner[29] = 0b00000001;

        let mut remover = GroupPositionRemover {
            side,
            bitmap_group,
            last_outer_index: Some(outer_index),
            group_position: GroupPosition::initial_for_side(side),
            pending_write: false,
        };

        let best_market_price = Ticks::from_indices(outer_index, InnerIndex::new(30));

        // Remove bit at inner index 29
        load_and_deactivate(
            &mut remover,
            GroupPosition {
                inner_index: InnerIndex::new(29),
                resting_order_index: RestingOrderIndex::new(0),
            },
            best_market_price,
        );
        assert_eq!(remover.pending_write, true);

        // No change in best price
        let new_best_market_price = remover
            .get_best_price_in_current(Some(best_market_price.inner_index()))
            .unwrap();
        assert_eq!(new_best_market_price, best_market_price);
    }

    #[test]
    pub fn test_remove_in_non_outermost_group_bid() {
        let side = Side::Bid;
        let outer_index = OuterIndex::new(1);

        let best_market_price = Ticks::from_indices(OuterIndex::new(2), InnerIndex::new(31));

        // Write initial data to slot
        let mut bitmap_group = BitmapGroup::default();
        bitmap_group.inner[30] = 0b10000001;
        bitmap_group.inner[29] = 0b00000001;

        let mut remover = GroupPositionRemover {
            side,
            bitmap_group,
            last_outer_index: Some(outer_index),
            group_position: GroupPosition::initial_for_side(side),
            pending_write: false,
        };

        // 1. Remove first bit on outermost
        load_and_deactivate(
            &mut remover,
            GroupPosition {
                inner_index: InnerIndex::new(30),
                resting_order_index: RestingOrderIndex::new(0),
            },
            best_market_price,
        );
        assert_eq!(remover.pending_write, true);

        // 2. Remove final bit on outermost
        load_and_deactivate(
            &mut remover,
            GroupPosition {
                inner_index: InnerIndex::new(30),
                resting_order_index: RestingOrderIndex::new(7),
            },
            best_market_price,
        );
        assert_eq!(remover.pending_write, true);

        // 3. Close the last bit in group
        load_and_deactivate(
            &mut remover,
            GroupPosition {
                inner_index: InnerIndex::new(29),
                resting_order_index: RestingOrderIndex::new(0),
            },
            best_market_price,
        );
        assert_eq!(remover.pending_write, false);
    }
}
