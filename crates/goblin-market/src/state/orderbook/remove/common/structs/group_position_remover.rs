use crate::{
    quantities::Ticks,
    state::{
        bitmap_group::BitmapGroup,
        iterator::active_position::active_group_position_iterator::ActiveGroupPositionIterator,
        order::group_position::GroupPosition,
        remove::{
            IGroupPositionLookupRemover, IGroupPositionRemover, IGroupPositionSequentialRemover,
        },
        ArbContext, OuterIndex, RestingOrderIndex, Side, TickIndices,
    },
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
        let index = 0;

        self.inner = ActiveGroupPositionIterator::new(bitmap_group, side, index);
    }

    fn load_outermost_group(&mut self, ctx: &mut ArbContext, best_market_price: Ticks) {
        let TickIndices {
            outer_index,
            inner_index,
        } = best_market_price.to_indices();

        let bitmap_group = BitmapGroup::new_from_slot(ctx, outer_index);
        let side = self.side();

        let starting_position = GroupPosition {
            inner_index,
            resting_order_index: RestingOrderIndex::ZERO,
        };
        let index = starting_position.index_inclusive(side);

        self.inner = ActiveGroupPositionIterator::new(bitmap_group, side, index);
    }

    fn write_to_slot(&self, ctx: &mut ArbContext, outer_index: OuterIndex) {
        self.inner.bitmap_group.write_to_slot(ctx, &outer_index);
    }

    fn side(&self) -> Side {
        self.inner.group_position_iterator.side
    }
}

impl IGroupPositionSequentialRemover for GroupPositionRemover {
    fn last_group_position(&self) -> Option<GroupPosition> {
        self.inner.group_position_iterator.last_group_position()
    }

    fn next(&mut self) -> Option<GroupPosition> {
        if let Some(group_position) = self.last_group_position() {
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
    fn find(&mut self, group_position: GroupPosition) -> bool {
        self.inner.find(group_position)
    }

    fn remove(&mut self) {
        if let Some(group_position) = self.group_position() {
            self.inner.bitmap_group.deactivate(group_position);
        }
    }

    fn group_position(&self) -> Option<GroupPosition> {
        self.inner.group_position_iterator.group_position()
    }

    fn increment_group_position(&mut self) {
        if self.inner.group_position_iterator.index < 255 {
            self.inner.group_position_iterator.index += 1;
        } else {
            self.inner.group_position_iterator.finished = true;
        }
    }

    fn decrement_group_position(&mut self) {
        // When GroupPositionIterator::next() is called on an empty group, a wrapping
        // add will turn index = 0 and set finished = true.
        // Decrement only when `finished` is false. In the false case all bitmap groups
        // were read but no active position was found.

        if !self.is_finished() {
            self.inner.group_position_iterator.index -= 1;
        }
    }

    // TODO remove
    fn is_only_active_bit_on_tick(&self, group_position: GroupPosition) -> bool {
        self.inner
            .bitmap_group
            .is_only_active_bit_on_tick(group_position)
    }

    fn is_lowest_active_bit_on_tick(&self, group_position: GroupPosition) -> bool {
        self.inner
            .bitmap_group
            .is_lowest_active_bit_on_tick(group_position)
    }

    fn is_group_active(&self) -> bool {
        self.inner.bitmap_group.is_group_active()
    }
}
