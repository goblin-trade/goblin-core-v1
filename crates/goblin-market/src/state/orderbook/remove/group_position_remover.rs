use crate::{
    quantities::Ticks,
    state::{
        bitmap_group::BitmapGroup,
        order::{group_position::GroupPosition, order_id::OrderId},
        ArbContext, InnerIndex, MarketPrices, OuterIndex, RestingOrderIndex, Side,
    },
};

/// Facilitates efficient batch deactivations in a bitmap group
pub struct GroupPositionRemover {
    // TODO use ActiveGroupPositionIterator as inner
    /// Whether for bids or asks
    /// Traverse upwards (ascending) for asks and downwards (descending) for bids
    pub side: Side,

    /// The current bitmap group pending a write. This allows us to perform multiple
    /// updates in a bitmap group with a single slot load. This value is written to slot
    /// when a new outer index is encountered.
    pub bitmap_group: BitmapGroup,

    /// The current group position used to paginate and for deactivate bits.
    /// The bit at group_position can either be active or inactive.
    pub group_position: GroupPosition,
}

impl GroupPositionRemover {
    pub fn new(side: Side) -> Self {
        GroupPositionRemover {
            side,
            bitmap_group: BitmapGroup::default(),

            // Default group position starts at the starting position of a given side
            group_position: GroupPosition::initial_for_side(side),
        }
    }

    /// Loads a new bitmap group for the new outer index. The previous group is flushed.
    ///
    /// # Externally ensure that
    ///
    /// * we always move away from the centre
    /// * it is not repeated for the same outer index
    ///
    pub fn load_outer_index(&mut self, ctx: &mut ArbContext, outer_index: OuterIndex) {
        self.bitmap_group = BitmapGroup::new_from_slot(ctx, outer_index);
        self.group_position = GroupPosition::initial_for_side(self.side);
    }

    /// Paginates to the given position and tells whether the bit is active
    ///
    /// Externally ensure that load_outer_index() was called first otherwise
    /// this will give a dummy value
    ///
    pub fn paginate_and_check_if_active(&mut self, group_position: GroupPosition) -> bool {
        self.group_position = group_position;
        self.bitmap_group.is_position_active(group_position)
    }

    /// Get the best active position in the group. Returns None if no active
    /// position is present from the starting from the current group position and onwards
    pub fn best_active_group_position(&self) -> Option<GroupPosition> {
        self.bitmap_group
            .best_active_group_position(self.side, self.group_position)
    }

    /// Try traversing to the next active group position in the current bitmap group.
    /// If an active position is present, updates `group_position` and returns true.
    /// Else returns false.
    pub fn try_traverse_to_best_active_position(&mut self) -> bool {
        if let Some(group_position) = self.best_active_group_position() {
            self.group_position = group_position;

            return true;
        }

        false
    }

    /// Whether the inner index has active ticks
    pub fn is_inner_index_active(&self, inner_index: InnerIndex) -> bool {
        self.bitmap_group.is_inner_index_active(inner_index)
    }

    /// Deactivate the bit at the current group position
    pub fn deactivate(&mut self) {
        self.bitmap_group.deactivate(self.group_position);
    }

    /// Whether the bitmap group has been inactivated for `self.side`. It accounts for
    /// and excludes bits belonging to the opposite side during lookup.
    ///
    /// Externally ensure that `last_outer_index` is not None and has active bits for `side`, so there is
    /// no overflow or underflow when we add or subtract from `best_opposite_inner_index`.
    ///
    /// This builds ActiveInnerIndexIterator and calls .next()
    /// TODO use inner: ActiveGroupPositionIterator. The opposite best price should be
    /// passed during initialization. ActiveInnerIndexIterator is redundant.
    ///
    /// Externally ensure that garbage bits are removed
    ///
    /// # Arguments
    ///
    /// * `best_opposite_price`
    ///
    pub fn is_group_inactive(&self, best_opposite_price: Ticks, outer_index: OuterIndex) -> bool {
        // TODO pass start_index_inclusive externally
        let start_index_inclusive = if outer_index == best_opposite_price.outer_index() {
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

        self.bitmap_group
            .is_inactive(self.side, start_index_inclusive)
    }

    pub fn is_group_inactive_v2(&self) -> bool {
        self.best_active_group_position().is_none()
    }

    // /// Flush the cached bitmap group to slot
    // ///
    // /// This should be called before moving to a new outer index
    // ///
    // /// # Arguments
    // ///
    // /// * `ctx`
    // ///
    // pub fn flush_bitmap_group(&mut self, ctx: &mut ArbContext, outer_index: OuterIndex) {
    //     if !self.pending_write {
    //         return;
    //     }

    //     self.bitmap_group.write_to_slot(ctx, &outer_index);
    //     self.pending_write = false;
    // }
}

// #[cfg(test)]
// mod tests {
//     use crate::state::RestingOrderIndex;

//     use super::*;

//     fn lookup_and_deactivate(
//         remover: &mut GroupPositionRemover,
//         group_position: GroupPosition,
//         best_market_price: Ticks,
//     ) {
//         let present = remover.is_position_active(group_position);
//         assert_eq!(present, true);

//         remover.deactivate_current_group_position(best_market_price);
//         let present_after_deactivation = remover.bitmap_group.is_position_active(group_position);
//         assert_eq!(present_after_deactivation, false);
//     }

//     // Test cases where cleared bitmap group is not written (pending_write is false)
//     // - Outermost tick closed
//     // - Whole group closed
//     //
//     // Cases where pending_write is true
//     // - Remove from outermost but last price does not change
//     // - Remove behind outermost

//     #[test]
//     pub fn test_pending_write_behavior_on_sequential_removals_ask() {
//         let side = Side::Ask;
//         let outer_index = OuterIndex::new(1);

//         // Write initial data to slot
//         let mut bitmap_group = BitmapGroup::default();
//         // Best opposite price
//         bitmap_group.inner[0] = 0b00000001;

//         // Best market price- two resting orders
//         bitmap_group.inner[1] = 0b10000001;

//         // Next inner price. Rest of the group is empty
//         bitmap_group.inner[2] = 0b00000001;

//         let mut remover = GroupPositionRemover {
//             side,
//             bitmap_group,
//             // last_outer_index: Some(outer_index),
//             group_position: GroupPosition::initial_for_side(side),
//             pending_write: false,
//         };

//         let mut best_market_price = Ticks::from_indices(outer_index, InnerIndex::ONE);

//         // 1. Remove first bit on outermost tick.
//         // - Since the tick remains active `pending_write` is true.
//         // - Best price does not update
//         lookup_and_deactivate(
//             &mut remover,
//             GroupPosition {
//                 inner_index: InnerIndex::ONE,
//                 resting_order_index: RestingOrderIndex::ZERO,
//             },
//             best_market_price,
//         );
//         assert_eq!(remover.pending_write, true);

//         // No change in market price
//         best_market_price = remover
//             .get_best_price_in_current(Some(InnerIndex::ONE))
//             .unwrap();
//         assert_eq!(
//             best_market_price,
//             Ticks::from_indices(outer_index, InnerIndex::ONE)
//         );

//         // 2. Remove final bit on the outermost tick
//         // - The outermost tick is closed so `pending_write` is false
//         // - Best price updated
//         lookup_and_deactivate(
//             &mut remover,
//             GroupPosition {
//                 inner_index: InnerIndex::ONE,
//                 resting_order_index: RestingOrderIndex::new(7),
//             },
//             best_market_price,
//         );
//         assert_eq!(remover.pending_write, false);

//         // Market price changed
//         best_market_price = remover
//             .get_best_price_in_current(Some(best_market_price.inner_index()))
//             .unwrap();
//         assert_eq!(
//             best_market_price,
//             Ticks::from_indices(outer_index, InnerIndex::new(2))
//         );

//         // 3. Remove final bit in the group
//         // - The whole group is closed so `pending_write` is false
//         // - Best price updated
//         lookup_and_deactivate(
//             &mut remover,
//             GroupPosition {
//                 inner_index: InnerIndex::new(2),
//                 resting_order_index: RestingOrderIndex::new(0),
//             },
//             best_market_price,
//         );
//         assert_eq!(remover.pending_write, false);

//         // Best price is None because the group is cleared
//         assert_eq!(
//             remover.get_best_price_in_current(Some(best_market_price.inner_index())),
//             None
//         );
//     }

//     #[test]
//     pub fn test_pending_write_behavior_on_removing_behind_best_price_ask() {
//         let side = Side::Ask;
//         let outer_index = OuterIndex::new(1);

//         // Write initial data to slot
//         let mut bitmap_group = BitmapGroup::default();
//         // Best opposite price
//         bitmap_group.inner[0] = 0b00000001;

//         // Best market price- two resting orders
//         bitmap_group.inner[1] = 0b10000001;

//         // Next inner price. Rest of the group is empty
//         bitmap_group.inner[2] = 0b00000001;

//         let mut remover = GroupPositionRemover {
//             side,
//             bitmap_group,
//             last_outer_index: Some(outer_index),
//             group_position: GroupPosition::initial_for_side(side),
//             pending_write: false,
//         };

//         let best_market_price = Ticks::from_indices(outer_index, InnerIndex::ONE);

//         // 3. Remove the bit at inner index 2
//         // - Best price is unchanged because ticks at best tick are not removed
//         // - `pending_write` is true
//         lookup_and_deactivate(
//             &mut remover,
//             GroupPosition {
//                 inner_index: InnerIndex::new(2),
//                 resting_order_index: RestingOrderIndex::new(0),
//             },
//             best_market_price,
//         );
//         assert_eq!(remover.pending_write, true);

//         // No change in best price
//         let new_best_market_price = remover
//             .get_best_price_in_current(Some(best_market_price.inner_index()))
//             .unwrap();
//         assert_eq!(new_best_market_price, best_market_price,);
//     }

//     #[test]
//     pub fn test_remove_in_non_outermost_group_ask() {
//         let side = Side::Ask;
//         let outer_index = OuterIndex::new(1);

//         let best_market_price = Ticks::from_indices(OuterIndex::new(2), InnerIndex::new(0));

//         // Write initial data to slot
//         let mut bitmap_group = BitmapGroup::default();
//         bitmap_group.inner[1] = 0b10000001;
//         bitmap_group.inner[2] = 0b00000001;

//         let mut remover = GroupPositionRemover {
//             side,
//             bitmap_group,
//             last_outer_index: Some(outer_index),
//             group_position: GroupPosition::initial_for_side(side),
//             pending_write: false,
//         };

//         // 1. Remove first bit on outermost
//         // - `pending_write` is true because group did not clear. The best price change
//         // condition does not apply on non-outermost groups
//         lookup_and_deactivate(
//             &mut remover,
//             GroupPosition {
//                 inner_index: InnerIndex::new(1),
//                 resting_order_index: RestingOrderIndex::new(0),
//             },
//             best_market_price,
//         );
//         assert_eq!(remover.pending_write, true);

//         // 2. Remove final bit on outermost
//         // - `pending_write` is true because group did not clear
//         lookup_and_deactivate(
//             &mut remover,
//             GroupPosition {
//                 inner_index: InnerIndex::new(1),
//                 resting_order_index: RestingOrderIndex::new(7),
//             },
//             best_market_price,
//         );
//         assert_eq!(remover.pending_write, true);

//         // 3. Close the last bit in group
//         // - `pending_write` is false because the whole group cleared
//         lookup_and_deactivate(
//             &mut remover,
//             GroupPosition {
//                 inner_index: InnerIndex::new(2),
//                 resting_order_index: RestingOrderIndex::new(0),
//             },
//             best_market_price,
//         );
//         assert_eq!(remover.pending_write, false);
//     }

//     #[test]
//     pub fn test_pending_write_behavior_on_sequential_removals_bid() {
//         let side = Side::Bid;
//         let outer_index = OuterIndex::new(1);

//         // Write initial data to slot
//         let mut bitmap_group = BitmapGroup::default();
//         // Best opposite price
//         bitmap_group.inner[31] = 0b00000001;

//         // Best market price - two resting orders
//         bitmap_group.inner[30] = 0b10000001;

//         // Next inner price. Rest of the group is empty
//         bitmap_group.inner[29] = 0b00000001;

//         let mut remover = GroupPositionRemover {
//             side,
//             bitmap_group,
//             last_outer_index: Some(outer_index),
//             group_position: GroupPosition::initial_for_side(side),
//             pending_write: false,
//         };

//         let mut best_market_price = Ticks::from_indices(outer_index, InnerIndex::new(30));

//         // 1. Remove first bit on outermost tick.
//         lookup_and_deactivate(
//             &mut remover,
//             GroupPosition {
//                 inner_index: InnerIndex::new(30),
//                 resting_order_index: RestingOrderIndex::ZERO,
//             },
//             best_market_price,
//         );
//         assert_eq!(remover.pending_write, true);

//         // No change in market price
//         best_market_price = remover
//             .get_best_price_in_current(Some(InnerIndex::new(30)))
//             .unwrap();
//         assert_eq!(
//             best_market_price,
//             Ticks::from_indices(outer_index, InnerIndex::new(30))
//         );

//         // 2. Remove final bit on the outermost tick
//         lookup_and_deactivate(
//             &mut remover,
//             GroupPosition {
//                 inner_index: InnerIndex::new(30),
//                 resting_order_index: RestingOrderIndex::new(7),
//             },
//             best_market_price,
//         );
//         assert_eq!(remover.pending_write, false);

//         // Market price changed
//         best_market_price = remover
//             .get_best_price_in_current(Some(best_market_price.inner_index()))
//             .unwrap();
//         assert_eq!(
//             best_market_price,
//             Ticks::from_indices(outer_index, InnerIndex::new(29))
//         );

//         // 3. Remove final bit in the group
//         lookup_and_deactivate(
//             &mut remover,
//             GroupPosition {
//                 inner_index: InnerIndex::new(29),
//                 resting_order_index: RestingOrderIndex::new(0),
//             },
//             best_market_price,
//         );
//         assert_eq!(remover.pending_write, false);

//         // Best price is None because the group is cleared
//         assert_eq!(
//             remover.get_best_price_in_current(Some(best_market_price.inner_index())),
//             None
//         );
//     }

//     #[test]
//     pub fn test_pending_write_behavior_on_removing_behind_best_price_bid() {
//         let side = Side::Bid;
//         let outer_index = OuterIndex::new(1);

//         // Write initial data to slot
//         let mut bitmap_group = BitmapGroup::default();
//         // Best opposite price
//         bitmap_group.inner[31] = 0b00000001;

//         // Best market price - two resting orders
//         bitmap_group.inner[30] = 0b10000001;

//         // Next inner price. Rest of the group is empty
//         bitmap_group.inner[29] = 0b00000001;

//         let mut remover = GroupPositionRemover {
//             side,
//             bitmap_group,
//             last_outer_index: Some(outer_index),
//             group_position: GroupPosition::initial_for_side(side),
//             pending_write: false,
//         };

//         let best_market_price = Ticks::from_indices(outer_index, InnerIndex::new(30));

//         // Remove bit at inner index 29
//         lookup_and_deactivate(
//             &mut remover,
//             GroupPosition {
//                 inner_index: InnerIndex::new(29),
//                 resting_order_index: RestingOrderIndex::new(0),
//             },
//             best_market_price,
//         );
//         assert_eq!(remover.pending_write, true);

//         // No change in best price
//         let new_best_market_price = remover
//             .get_best_price_in_current(Some(best_market_price.inner_index()))
//             .unwrap();
//         assert_eq!(new_best_market_price, best_market_price);
//     }

//     #[test]
//     pub fn test_remove_in_non_outermost_group_bid() {
//         let side = Side::Bid;
//         let outer_index = OuterIndex::new(1);

//         let best_market_price = Ticks::from_indices(OuterIndex::new(2), InnerIndex::new(31));

//         // Write initial data to slot
//         let mut bitmap_group = BitmapGroup::default();
//         bitmap_group.inner[30] = 0b10000001;
//         bitmap_group.inner[29] = 0b00000001;

//         let mut remover = GroupPositionRemover {
//             side,
//             bitmap_group,
//             last_outer_index: Some(outer_index),
//             group_position: GroupPosition::initial_for_side(side),
//             pending_write: false,
//         };

//         // 1. Remove first bit on outermost
//         lookup_and_deactivate(
//             &mut remover,
//             GroupPosition {
//                 inner_index: InnerIndex::new(30),
//                 resting_order_index: RestingOrderIndex::new(0),
//             },
//             best_market_price,
//         );
//         assert_eq!(remover.pending_write, true);

//         // 2. Remove final bit on outermost
//         lookup_and_deactivate(
//             &mut remover,
//             GroupPosition {
//                 inner_index: InnerIndex::new(30),
//                 resting_order_index: RestingOrderIndex::new(7),
//             },
//             best_market_price,
//         );
//         assert_eq!(remover.pending_write, true);

//         // 3. Close the last bit in group
//         lookup_and_deactivate(
//             &mut remover,
//             GroupPosition {
//                 inner_index: InnerIndex::new(29),
//                 resting_order_index: RestingOrderIndex::new(0),
//             },
//             best_market_price,
//         );
//         assert_eq!(remover.pending_write, false);
//     }
// }
