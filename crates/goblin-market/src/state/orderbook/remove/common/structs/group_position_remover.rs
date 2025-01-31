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
    pub active_group_position_iterator: ActiveGroupPositionIterator,
}

impl GroupPositionRemover {
    /// Initialize a new group position remover
    ///
    /// # Arguments
    ///
    /// * `side`
    pub fn new(side: Side) -> Self {
        GroupPositionRemover {
            active_group_position_iterator: ActiveGroupPositionIterator::new_default_for_side(side),
        }
    }
}

impl IGroupPositionRemover for GroupPositionRemover {
    fn load_outer_index(&mut self, ctx: &ArbContext, outer_index: OuterIndex) {
        let bitmap_group = BitmapGroup::new_from_slot(ctx, outer_index);
        let side = self.side();
        let index = 0;

        self.active_group_position_iterator =
            ActiveGroupPositionIterator::new(bitmap_group, side, index);
    }

    fn bitmap_group_mut(&mut self) -> &mut BitmapGroup {
        &mut self.active_group_position_iterator.bitmap_group
    }

    fn side(&self) -> Side {
        self.active_group_position_iterator
            .group_position_iterator
            .side
    }

    fn current_position(&self) -> Option<GroupPosition> {
        self.active_group_position_iterator
            .group_position_iterator
            .current_position()
    }
}

impl IGroupPositionSequentialRemover for GroupPositionRemover {
    fn next(&mut self) -> Option<GroupPosition> {
        if let Some(group_position) = self.current_position() {
            self.active_group_position_iterator
                .bitmap_group
                .deactivate(group_position);
        }

        // If the group has no active positions, the inner iterator will traverse
        // to the last position and mark itself as finished
        self.active_group_position_iterator.next()
    }

    fn is_uninitialized_or_exhausted(&self) -> bool {
        self.is_uninitialized() || self.is_exhausted()
    }

    fn is_uninitialized(&self) -> bool {
        self.active_group_position_iterator
            .group_position_iterator
            .next_index
            == 0
    }

    fn is_exhausted(&self) -> bool {
        self.active_group_position_iterator
            .group_position_iterator
            .exhausted
    }

    fn load_outermost_group(&mut self, ctx: &ArbContext, best_market_price: Ticks) {
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
        let index = starting_position.bit_index(side);

        self.active_group_position_iterator =
            ActiveGroupPositionIterator::new(bitmap_group, side, index);
    }
}

impl IGroupPositionLookupRemover for GroupPositionRemover {
    fn visit_and_check_if_active(&mut self, group_position: GroupPosition) -> bool {
        self.active_group_position_iterator.find(group_position)
    }

    fn deactivate_current(&mut self) {
        if let Some(group_position) = self.current_position() {
            self.active_group_position_iterator
                .bitmap_group
                .deactivate(group_position);
        }
    }
}
