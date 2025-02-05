use crate::state::{
    bitmap_group::BitmapGroup,
    iterator::position::{BitIndexIterator, GroupPositionIterator},
    order::group_position::GroupPosition,
    remove::{GroupPositionLookupRemover, GroupPositionSequentialRemover},
    BitIndex, RestingOrderIndex, Side, TickIndices,
};

/// Iterates through active positions in a bitmap group
///
/// Mappings for new iterator
/// - current / previous remains current
/// - Use peek() to get next value without iterating
///
/// - Instead of setting value of next(), set the value of current as one position behind next
/// TODO check where are we setting next()?
pub struct ActiveGroupPositionIterator {
    pub bitmap_group: BitmapGroup,
    pub group_position_iterator: GroupPositionIterator,
}

impl ActiveGroupPositionIterator {
    pub fn new(side: Side) -> Self {
        ActiveGroupPositionIterator {
            bitmap_group: BitmapGroup::default(),
            group_position_iterator: GroupPositionIterator {
                side,
                bit_index_iterator: BitIndexIterator {
                    current_index: None,
                },
            },
        }
    }

    pub fn side(&self) -> crate::state::Side {
        self.group_position_iterator.side
    }

    pub fn load_outer_index(
        &mut self,
        ctx: &crate::state::ArbContext,
        outer_index: crate::state::OuterIndex,
    ) {
        self.bitmap_group = BitmapGroup::new_from_slot(ctx, outer_index);

        // The current_index is set to 0. Calling next() will give 1 or a bigger bit index
        // is_uninitialized is false
        let bit_index = BitIndex::new(0);
        self.group_position_iterator
            .bit_index_iterator
            .set_current_index(Some(bit_index));
    }

    pub fn current_position(&self) -> Option<GroupPosition> {
        self.group_position_iterator.current_position()
    }

    pub fn bitmap_group_mut(&mut self) -> &mut BitmapGroup {
        &mut self.bitmap_group
    }
}

impl Iterator for ActiveGroupPositionIterator {
    type Item = GroupPosition;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(next_group_position) = self.group_position_iterator.next() {
            if self.bitmap_group.is_position_active(next_group_position) {
                return Some(next_group_position);
            }
        }
        None
    }
}

impl GroupPositionSequentialRemover for ActiveGroupPositionIterator {
    fn deactivate_previous_and_get_next(&mut self) -> Option<GroupPosition> {
        if let Some(group_position) = self.current_position() {
            self.bitmap_group.deactivate(group_position);
        }

        // If the group has no active positions, the inner iterator will traverse
        // to the last position and mark itself as finished
        self.next()
    }

    fn is_uninitialized(&self) -> bool {
        self.group_position_iterator
            .bit_index_iterator
            .current_index
            .is_none()
    }

    fn is_exhausted(&self) -> bool {
        self.group_position_iterator
            .bit_index_iterator
            .current_index
            == Some(BitIndex::MAX)
    }

    fn is_uninitialized_or_exhausted(&self) -> bool {
        self.is_uninitialized() || self.is_exhausted()
    }

    fn load_outermost_group(
        &mut self,
        ctx: &crate::state::ArbContext,
        best_market_price: crate::quantities::Ticks,
    ) {
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
        let bit_index = BitIndex::from((side, starting_position));

        self.bitmap_group = bitmap_group;
        self.group_position_iterator
            .bit_index_iterator
            .set_current_index(Some(bit_index));
    }
}

impl GroupPositionLookupRemover for ActiveGroupPositionIterator {
    fn visit_and_check_if_active(&mut self, group_position: GroupPosition) -> bool {
        self.group_position_iterator
            .set_current_position(Some(group_position));

        self.bitmap_group.is_position_active(group_position)
    }

    fn deactivate_current(&mut self) {
        if let Some(group_position) = self.group_position_iterator.current_position() {
            self.bitmap_group.deactivate(group_position);
        }
    }
}
