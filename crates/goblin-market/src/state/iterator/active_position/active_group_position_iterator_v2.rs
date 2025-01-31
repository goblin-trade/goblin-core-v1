use crate::state::{
    bitmap_group::BitmapGroup,
    iterator::position::{BitIndexIterator, GroupPositionIteratorV2},
    order::group_position::GroupPosition,
    remove::{IGroupPositionLookupRemover, IGroupPositionRemover},
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

// TODO move function in a trait. lookup_if_active() is only used by lookup remover.
// Use the nested group_position_iterator to get current position
// impl ActiveGroupPositionIteratorV2 {
//     /// Visit the given position and check whether it holds an active order
//     pub fn visit_and_check_if_active(&mut self, group_position: GroupPosition) -> bool {
//         self.group_position_iterator
//             .set_current_position(Some(group_position));

//         self.bitmap_group.is_position_active(group_position)
//     }
// }

impl IGroupPositionRemover for ActiveGroupPositionIteratorV2 {
    fn side(&self) -> crate::state::Side {
        self.group_position_iterator.side
    }

    // TODO remove. Instead create a new ActiveGroupPosition instance
    fn load_outer_index(
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

    fn current_position(&self) -> Option<GroupPosition> {
        self.group_position_iterator.current_position()
    }

    fn bitmap_group_mut(&mut self) -> &mut BitmapGroup {
        &mut self.bitmap_group
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
