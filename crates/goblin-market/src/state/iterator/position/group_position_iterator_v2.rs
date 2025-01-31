use crate::state::{order::group_position::GroupPosition, Side};

use super::bit_index_iterator::BitIndexIterator;

pub struct GroupPositionIteratorV2 {
    pub side: Side,
    pub bit_index_iterator: BitIndexIterator,
}

impl GroupPositionIteratorV2 {
    pub fn position(&self) -> Option<GroupPosition> {
        if let Some(bit_index) = self.bit_index_iterator.current_index() {
            Some(GroupPosition::from_bit_index(self.side, bit_index))
        } else {
            None
        }
    }
}
