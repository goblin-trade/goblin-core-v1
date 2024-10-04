use crate::{
    quantities::Ticks,
    state::{
        bitmap_group::BitmapGroup,
        iterator::active_position::active_group_position_iterator::ActiveGroupPositionIterator,
        order::group_position::GroupPosition, ArbContext, InnerIndex, OuterIndex, Side,
    },
};

/// Facilitates efficient batch deactivations in a bitmap group
pub struct GroupPositionRemoverV2 {
    pub inner: ActiveGroupPositionIterator,
}

impl GroupPositionRemoverV2 {
    pub fn new(side: Side) -> Self {
        GroupPositionRemoverV2 {
            inner: ActiveGroupPositionIterator::new_default_for_side(side),
        }
    }

    pub fn load_outer_index(&mut self, ctx: &mut ArbContext, outer_index: OuterIndex) {
        let bitmap_group = BitmapGroup::new_from_slot(ctx, outer_index);
        let side = self.side();
        let count = 0;

        let new_iterator = ActiveGroupPositionIterator::new(bitmap_group, side, count);
        self.inner = new_iterator;
    }

    /// Paginates to the given position and tells whether the bit is active
    ///
    /// Externally ensure that load_outer_index() was called first otherwise
    /// this will give a dummy value
    ///
    pub fn paginate_and_check_if_active(&mut self, group_position: GroupPosition) -> bool {
        self.set_group_position(group_position);
        self.is_position_active(group_position)
    }

    // Breaking- pointer now moves to end of the group even if no active bit is present
    pub fn try_traverse_to_best_active_position(&mut self) -> Option<GroupPosition> {
        self.inner.next()
    }

    /// Deactivate the bit at the currently tracked group position
    ///
    /// Externally ensure that try_traverse_to_best_active_position() is called
    /// before deactivation
    pub fn deactivate(&mut self) {
        let group_position = self.group_position_unchecked();
        self.inner.bitmap_group.deactivate(group_position);
    }

    // TODO use best_market_price to test for inactivity.
    // This should remove the need to clean garbage bits
    //
    pub fn is_group_inactive(&self, best_opposite_price: Ticks, outer_index: OuterIndex) -> bool {
        // TODO pass start_index_inclusive externally
        let start_index_inclusive = if outer_index == best_opposite_price.outer_index() {
            // Overflow or underflow would happen only if the most extreme bitmap is occupied
            // by opposite side bits. This is not possible because active bits for `side`
            // are guaranteed to be present.

            let best_opposite_inner_index = best_opposite_price.inner_index();
            Some(if self.side() == Side::Bid {
                best_opposite_inner_index - InnerIndex::ONE
            } else {
                best_opposite_inner_index + InnerIndex::ONE
            })
        } else {
            None
        };

        self.inner
            .bitmap_group
            .is_inactive(self.side(), start_index_inclusive)
    }

    // Getters

    pub fn side(&self) -> Side {
        self.inner.group_position_iterator.side
    }

    fn is_position_active(&self, group_position: GroupPosition) -> bool {
        self.inner.bitmap_group.is_position_active(group_position)
    }

    /// Get the currently tracked group position
    ///
    /// Unsafe function- Externally ensure that try_traverse_to_best_active_position()
    /// is called before calling.
    fn group_position_unchecked(&self) -> GroupPosition {
        let group_position = self.inner.group_position_iterator.last_group_position();
        debug_assert!(group_position.is_some());
        let group_position_unchecked = unsafe { group_position.unwrap_unchecked() };

        group_position_unchecked
    }

    // Setters

    pub fn set_group_position(&mut self, group_position: GroupPosition) {
        let count = group_position.index_inclusive(self.side());
        self.inner.group_position_iterator.index = count;
    }

    pub fn set_count(&mut self) {
        // self.inner.
    }
}
