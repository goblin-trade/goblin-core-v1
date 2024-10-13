use crate::state::{
    bitmap_group::BitmapGroup,
    iterator::active_position::active_group_position_iterator::ActiveGroupPositionIterator,
    order::group_position::GroupPosition,
    remove::{IGroupPositionLookupRemover, IGroupPositionRemover, IGroupPositionSequentialRemover},
    ArbContext, OuterIndex, Side,
};

/// Facilitates efficient batch deactivations in a bitmap group
pub struct GroupPositionRemover {
    /// Iterator to read active positions in a bitmap group
    pub inner: ActiveGroupPositionIterator,
}

impl GroupPositionRemover {
    /// Initialize a new group position remover
    ///
    /// # Arguments
    ///
    /// * `side`
    pub fn new(side: Side) -> Self {
        GroupPositionRemover {
            inner: ActiveGroupPositionIterator::new_default_for_side(side),
        }
    }
}

impl IGroupPositionRemover for GroupPositionRemover {
    fn load_outer_index(&mut self, ctx: &mut ArbContext, outer_index: OuterIndex) {
        let bitmap_group = BitmapGroup::new_from_slot(ctx, outer_index);
        let side = self.side();
        let count = 0;

        self.inner = ActiveGroupPositionIterator::new(bitmap_group, side, count);
    }

    fn write_to_slot(&self, ctx: &mut ArbContext, outer_index: OuterIndex) {
        self.inner.bitmap_group.write_to_slot(ctx, &outer_index);
    }

    fn side(&self) -> Side {
        self.inner.group_position_iterator.side
    }

    fn group_position(&self) -> Option<GroupPosition> {
        self.inner.group_position_iterator.last_group_position()
    }
}

impl IGroupPositionSequentialRemover for GroupPositionRemover {
    fn next(&mut self) -> Option<GroupPosition> {
        if let Some(group_position) = self.group_position() {
            self.inner.bitmap_group.deactivate(group_position);
        }

        // If the group has no active positions, the inner iterator will traverse
        // to the last position and mark itself as finished
        self.inner.next()
    }

    fn is_uninitialized_or_finished(&self) -> bool {
        self.is_uninitialized() || self.is_finished()
    }

    fn is_uninitialized(&self) -> bool {
        self.inner.group_position_iterator.index == 0
    }

    fn is_finished(&self) -> bool {
        self.inner.group_position_iterator.finished
    }
}

impl IGroupPositionLookupRemover for GroupPositionRemover {
    fn paginate_and_check_if_active(&mut self, group_position: GroupPosition) -> bool {
        self.inner.paginate_and_check_if_active(group_position)
    }

    fn deactivate(&mut self, group_position: GroupPosition) {
        self.inner.bitmap_group.deactivate(group_position);
    }

    fn is_only_active_bit_on_tick(&self, group_position: GroupPosition) -> bool {
        self.inner
            .bitmap_group
            .is_only_active_bit_on_tick(group_position)
    }

    fn is_group_active(&self) -> bool {
        self.inner.bitmap_group.is_group_active()
    }
}
