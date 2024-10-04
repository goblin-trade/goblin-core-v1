use crate::{
    quantities::Ticks,
    state::{
        bitmap_group::BitmapGroup,
        iterator::active_position::active_group_position_iterator::ActiveGroupPositionIterator,
        order::{group_position::GroupPosition, order_id::OrderId},
        ArbContext, InnerIndex, MarketPrices, OuterIndex, RestingOrderIndex, Side,
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

    pub fn deactivate(&mut self) {
        // let group_position = self.inner.group_position_iterator.
    }

    // Getters

    pub fn side(&self) -> Side {
        self.inner.group_position_iterator.side
    }

    fn is_position_active(&self, group_position: GroupPosition) -> bool {
        self.inner.bitmap_group.is_position_active(group_position)
    }

    // Setters

    pub fn set_group_position(&mut self, group_position: GroupPosition) {
        let count = group_position.count_inclusive(self.side());
        self.inner.group_position_iterator.index = count;
    }

    pub fn set_count(&mut self) {
        // self.inner.
    }
}
