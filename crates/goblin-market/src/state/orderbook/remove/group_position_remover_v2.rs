use crate::state::{
    bitmap_group::BitmapGroup,
    iterator::active_position::active_group_position_iterator::ActiveGroupPositionIterator,
    order::group_position::GroupPosition, ArbContext, OuterIndex, Side,
};

/// Facilitates efficient batch deactivations in a bitmap group
pub struct GroupPositionRemoverV2 {
    /// Iterator to read active positions in a bitmap group
    pub inner: ActiveGroupPositionIterator,
}

impl GroupPositionRemoverV2 {
    /// Initialize a new group position remover
    ///
    /// # Arguments
    ///
    /// * `side`
    pub fn new(side: Side) -> Self {
        GroupPositionRemoverV2 {
            inner: ActiveGroupPositionIterator::new_default_for_side(side),
        }
    }

    /// Load bitmap group for the given outer index
    ///
    /// # Arguments
    ///
    /// * `ctx` - Context to read from slot
    /// * `outer_index` - Load bitmap group for this outer index
    pub fn load_outer_index(&mut self, ctx: &mut ArbContext, outer_index: OuterIndex) {
        let bitmap_group = BitmapGroup::new_from_slot(ctx, outer_index);
        let side = self.side();
        let count = 0;

        self.inner = ActiveGroupPositionIterator::new(bitmap_group, side, count);
    }

    /// Write the bitmap group to slot
    pub fn write_to_slot(&self, ctx: &mut ArbContext, outer_index: OuterIndex) {
        self.inner.bitmap_group.write_to_slot(ctx, &outer_index);
    }

    /// Get side for this remover
    pub fn side(&self) -> Side {
        self.inner.group_position_iterator.side
    }
}

pub trait SequentialGroupPositionRemover {
    fn deactivate_current_and_get_next(&mut self) -> Option<GroupPosition>;
    fn group_position(&self) -> Option<GroupPosition>;
    fn is_uninitialized_or_finished(&self) -> bool;
}

impl SequentialGroupPositionRemover for GroupPositionRemoverV2 {
    fn deactivate_current_and_get_next(&mut self) -> Option<GroupPosition> {
        if let Some(group_position) = self.group_position() {
            self.inner.bitmap_group.deactivate(group_position);
        }

        // If the group has no active positions, the inner iterator will traverse
        // to the last position and mark itself as finished
        self.inner.next()
    }

    /// Get the current group position if it is loaded
    fn group_position(&self) -> Option<GroupPosition> {
        self.inner.group_position_iterator.last_group_position()
    }

    /// Whether the group is uninitialized or whether reads are finished
    fn is_uninitialized_or_finished(&self) -> bool {
        self.inner.group_position_iterator.index == 0 || self.inner.group_position_iterator.finished
    }
}

pub trait RandomGroupPositionRemover {
    fn paginate_and_check_if_active(&mut self, group_position: GroupPosition) -> bool;
    fn deactivate(&mut self, group_position: GroupPosition);
    fn is_only_active_bit_on_tick(&self, group_position: GroupPosition) -> bool;
    fn is_group_active(&self) -> bool;
}

impl RandomGroupPositionRemover for GroupPositionRemoverV2 {
    // Setters

    /// Paginates to the given position and check whether the bit is active
    ///
    /// Externally ensure that load_outer_index() was called first otherwise
    /// this will give a blank value.
    fn paginate_and_check_if_active(&mut self, group_position: GroupPosition) -> bool {
        self.inner.paginate_and_check_if_active(group_position)
    }

    /// Deactivate the bit at the currently tracked group position
    ///
    /// Externally ensure that try_traverse_to_best_active_position() is called
    /// before deactivation
    fn deactivate(&mut self, group_position: GroupPosition) {
        self.inner.bitmap_group.deactivate(group_position);
    }

    // Getters

    /// Whether `group_position` holds the only active bit on its corresponding
    /// inner index and by extension price
    fn is_only_active_bit_on_tick(&self, group_position: GroupPosition) -> bool {
        self.inner
            .bitmap_group
            .is_only_active_bit_on_tick(group_position)
    }

    /// Whether the current bitmap group has any active positions
    fn is_group_active(&self) -> bool {
        self.inner.bitmap_group.is_group_active()
    }
}

// TODO tests
