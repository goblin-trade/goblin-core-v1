use crate::{
    quantities::Ticks,
    state::{
        bitmap_group::BitmapGroup,
        order::{group_position::GroupPosition, order_id::OrderId},
        InnerIndex, OuterIndex, Side, SlotStorage,
    },
};

/// Facilitates efficient batch deactivations at GroupPositions
pub struct GroupPositionRemover {
    /// Whether for bids or asks
    /// Traverse upwards (ascending) for asks and downwards (descending) for bids
    pub side: Side,

    /// The current bitmap group pending a write. This allows us to perform multiple
    /// updates in a bitmap group with a single slot load. This value is written to slot
    /// when a new outer index is encountered.
    pub bitmap_group: BitmapGroup,

    /// Outer index corresponding to `bitmap_group`
    pub last_outer_index: Option<OuterIndex>,

    /// The last searched group position. Used to re-construct the last searched order id
    pub last_searched_group_position: Option<GroupPosition>,

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
            last_searched_group_position: None,
            pending_write: false,
        }
    }

    /// The last searched order ID
    pub fn last_searched_order_id(&self) -> Option<OrderId> {
        if let (Some(outer_index), Some(group_position)) =
            (self.last_outer_index, self.last_searched_group_position)
        {
            return Some(OrderId {
                price_in_ticks: Ticks::from_indices(outer_index, group_position.inner_index),
                resting_order_index: group_position.resting_order_index,
            });
        }
        None
    }

    /// Whether a resting order is present at given (inner_index, resting_order_index)
    ///
    /// Externally ensure that load_outer_index() was called first so that
    /// `last_outer_index` is not None
    ///
    pub fn order_present(&mut self, group_position: GroupPosition) -> bool {
        self.last_searched_group_position = Some(group_position);

        let GroupPosition {
            inner_index,
            resting_order_index,
        } = group_position;
        let bitmap = self.bitmap_group.get_bitmap(&inner_index);
        bitmap.order_present(resting_order_index)
    }

    /// Deactivates the bit present on `last_searched_group_position` and conditionally
    /// enables or disables `pending_write`
    ///
    /// Sets pending_write to false if the last bit on best market price is deactivated,
    /// else set to true. If the last bit on best maket price is deactivated there is no
    /// need to write the bitmap group to slot because the best market price will update.
    ///
    pub fn deactivate_last_searched_group_position(&mut self, best_market_price: Ticks) {
        if let (Some(outer_index), Some(group_position)) =
            (self.last_outer_index, self.last_searched_group_position)
        {
            let current_price = Ticks::from_indices(outer_index, group_position.inner_index);
            let mut bitmap = self
                .bitmap_group
                .get_bitmap_mut(&group_position.inner_index);
            bitmap.clear(&group_position.resting_order_index);

            // TODO pending_write should be false if group got cleared
            // self.bitmap_group.is_inactive(side, start_index_inclusive)
            self.pending_write = !(current_price == best_market_price && bitmap.is_empty());
            self.last_searched_group_position = None;

            // Optimization- no need to clear `last_searched_group_position`,
            // since `deactivate_in_current()` is not called without calling
            // `order_present()` first
        }
    }

    // /// Deactivate bit at the last searched group position
    // ///
    // /// Externally ensure that load_outer_index() was called first so that
    // /// `last_outer_index` is not None
    // ///
    // pub fn deactivate_last_searched_group_position(&mut self) {
    //     if let Some(group_position) = self.last_searched_group_position {
    //         let mut bitmap = self
    //             .bitmap_group
    //             .get_bitmap_mut(&group_position.inner_index);
    //         bitmap.clear(&group_position.resting_order_index);
    //         self.pending_write = true;

    //         // We need to clear last_searched_group_position so that behavior
    //         // remains consistent with slides.
    //         self.last_searched_group_position = None;

    //         // Optimization- no need to clear `last_searched_group_position`,
    //         // since `deactivate_in_current()` is not called without calling
    //         // `order_present()` first
    //     }
    // }

    pub fn deactivation_will_close_best_inner_index_v2(&self, best_market_price: Ticks) -> bool {
        if let Some(order_id) = self.last_searched_order_id() {
            if order_id.price_in_ticks == best_market_price {
                let group_position = self.last_searched_group_position.unwrap();
                let bitmap = self.bitmap_group.get_bitmap(&group_position.inner_index);
                return bitmap.will_be_cleared_after_removal(group_position.resting_order_index);
            }
        }
        false
    }

    pub fn deactivation_will_close_best_inner_index(&self, best_inner_index: InnerIndex) -> bool {
        if let Some(group_position) = self.last_searched_group_position {
            if group_position.inner_index == best_inner_index {
                let bitmap = self.bitmap_group.get_bitmap(&group_position.inner_index);
                return bitmap.will_be_cleared_after_removal(group_position.resting_order_index);
            }
        }

        false
    }

    /// Get price of the best active order in the current bitmap group,
    /// beginning from a given position
    ///
    /// # Arguments
    ///
    /// * `starting_index` - Search beginning from this index (inclusive) if Some,
    /// else begin lookup from the edge of the bitmap group.
    ///
    pub fn get_best_price_in_current(&self, starting_index: Option<InnerIndex>) -> Option<Ticks> {
        if let (Some(outer_index), Some(inner_index)) = (
            self.last_outer_index,
            self.bitmap_group
                .best_active_inner_index(self.side, starting_index),
        ) {
            Some(Ticks::from_indices(outer_index, inner_index))
        } else {
            None
        }
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
    pub fn is_inactive(&self, best_opposite_price: Ticks) -> bool {
        let start_index = if self.last_outer_index == Some(best_opposite_price.outer_index()) {
            // Overflow or underflow would happen only if the most extreme bitmap is occupied
            // by opposite side bits. This is not possible because active bits for `side`
            // are guaranteed to be present.

            let best_opposite_inner_index = best_opposite_price.inner_index();
            if self.side == Side::Bid {
                best_opposite_inner_index - InnerIndex::ONE
            } else {
                best_opposite_inner_index + InnerIndex::ONE
            }
        } else if self.side == Side::Bid {
            InnerIndex::MAX
        } else {
            InnerIndex::MIN
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
    pub fn load_outer_index(&mut self, slot_storage: &mut SlotStorage, outer_index: OuterIndex) {
        if self.last_outer_index == Some(outer_index) {
            return;
        }
        // Outer index changed. Flush the old bitmap group to slot.
        self.flush_bitmap_group(slot_storage);

        self.last_outer_index = Some(outer_index);
        self.last_searched_group_position = None;
        self.bitmap_group = BitmapGroup::new_from_slot(slot_storage, outer_index);
    }

    /// Flush the cached bitmap group to slot
    /// This should be called before moving to a new outer index
    pub fn flush_bitmap_group(&mut self, slot_storage: &mut SlotStorage) {
        if !self.pending_write {
            return;
        }

        if let Some(last_index) = self.last_outer_index {
            // Don't write to slot if
            // - Bitmap group was completely cleared. Since outer index is removed from list
            // we can infer an empty group from outer index.
            // - Group closed for side, but not opposite side- This means that the bitmap will
            // hold a ghost value that's not valid for the opposite side. The tick price of this
            // bit will be closer to the centre than best price stored in market state. We'll need
            // a way to ignore this value- form a virtual bitmap when inserting values that
            // only considers bits at best market price or inner.
            self.bitmap_group.write_to_slot(slot_storage, &last_index);
            self.pending_write = false;
        }
    }
}
