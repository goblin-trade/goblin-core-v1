use crate::state::{
    bitmap_group::BitmapGroup,
    iterator::position::{BitIndexIterator, GroupPositionIteratorV2},
    order::group_position::GroupPosition,
    remove::{IGroupPositionLookupRemover, IGroupPositionSequentialRemover},
    RestingOrderIndex, Side, TickIndices,
};

/// Iterates through active positions in a bitmap group
///
/// Mappings for new iterator
/// - current / previous remains current
/// - Use peek() to get next value without iterating
///
/// - Instead of setting value of next(), set the value of current as one position behind next
/// TODO check where are we setting next()?
pub struct ActiveGroupPositionIteratorV2 {
    pub bitmap_group: BitmapGroup,
    pub group_position_iterator: GroupPositionIteratorV2,
}

impl ActiveGroupPositionIteratorV2 {
    pub fn new(side: Side) -> Self {
        ActiveGroupPositionIteratorV2 {
            bitmap_group: BitmapGroup::default(),
            group_position_iterator: GroupPositionIteratorV2 {
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
        let bit_index = 0;
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

impl Iterator for ActiveGroupPositionIteratorV2 {
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

// impl IGroupPositionRemover for ActiveGroupPositionIteratorV2 {
//     fn side(&self) -> crate::state::Side {
//         self.group_position_iterator.side
//     }

//     fn load_outer_index(
//         &mut self,
//         ctx: &crate::state::ArbContext,
//         outer_index: crate::state::OuterIndex,
//     ) {
//         self.bitmap_group = BitmapGroup::new_from_slot(ctx, outer_index);

//         // The current_index is set to 0. Calling next() will give 1 or a bigger bit index
//         // is_uninitialized is false
//         let bit_index = 0;
//         self.group_position_iterator
//             .bit_index_iterator
//             .set_current_index(Some(bit_index));
//     }

//     fn current_position(&self) -> Option<GroupPosition> {
//         self.group_position_iterator.current_position()
//     }

//     fn bitmap_group_mut(&mut self) -> &mut BitmapGroup {
//         &mut self.bitmap_group
//     }
// }

impl IGroupPositionSequentialRemover for ActiveGroupPositionIteratorV2 {
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
            == Some(255)
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
        let index = starting_position.bit_index(side);

        self.bitmap_group = bitmap_group;
        self.group_position_iterator
            .bit_index_iterator
            .set_current_index(Some(index));
    }
}

impl IGroupPositionLookupRemover for ActiveGroupPositionIteratorV2 {
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
