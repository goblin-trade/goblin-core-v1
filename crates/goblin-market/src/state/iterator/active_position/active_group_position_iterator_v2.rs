use crate::state::{
    bitmap_group::BitmapGroup, iterator::position::GroupPositionIteratorV2,
    order::group_position::GroupPosition,
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

// TODO move function in a trait. lookup_if_active() is only used by lookup remover.
// Use the nested group_position_iterator to get current position
impl ActiveGroupPositionIteratorV2 {
    /// Visit the given position and check whether it holds an active order
    pub fn visit_and_check_if_active(&mut self, group_position: GroupPosition) -> bool {
        self.group_position_iterator
            .set_current_position(Some(group_position));

        self.bitmap_group.is_position_active(group_position)
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
