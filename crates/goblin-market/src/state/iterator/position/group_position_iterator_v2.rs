use crate::state::{order::group_position::GroupPosition, Side};

use super::bit_index_iterator::BitIndexIterator;

/// Iterate across group positions in a group, in the sequence of matching according
/// to the side. It is a thin layer on top of BitIndexIterator.
///
/// * For asks: Moves row-wise from resting order index 0 to 7, then bottom to top
/// from inner index 0 to 31
/// * For bids: Moves row-wise from resting order index 0 to 7, then top to bottom
/// from inner index 31 to 0
///
pub struct GroupPositionIteratorV2 {
    pub side: Side,
    pub bit_index_iterator: BitIndexIterator,
}

impl GroupPositionIteratorV2 {
    pub fn current_position(&self) -> Option<GroupPosition> {
        let bit_index = self.bit_index_iterator.current_index()?;
        Some(GroupPosition::from_bit_index(self.side, bit_index))
    }

    pub fn set_current_position(&mut self, position: Option<GroupPosition>) {
        let bit_index = position.map(|position| position.bit_index(self.side));
        self.bit_index_iterator.set_current_index(bit_index);
    }

    pub fn peek(&self) -> Option<GroupPosition> {
        let bit_index = self.bit_index_iterator.peek()?;
        Some(GroupPosition::from_bit_index(self.side, bit_index))
    }
}

impl Iterator for GroupPositionIteratorV2 {
    type Item = GroupPosition;

    fn next(&mut self) -> Option<Self::Item> {
        let bit_index = self.bit_index_iterator.next()?;
        Some(GroupPosition::from_bit_index(self.side, bit_index))
    }
}
