use crate::state::{order::group_position::GroupPosition, Side};

use super::bit_index_iterator::BitIndexIterator;

/// Iterate across consecutive group positions inside a bitmap group.
/// It is a thin layer on top of BitIndexIterator.
///
/// * For asks: Moves row-wise from resting order index 0 to 7, then bottom to top
/// from inner index 0 to 31
/// * For bids: Moves row-wise from resting order index 0 to 7, then top to bottom
/// from inner index 31 to 0
///
pub struct GroupPositionIterator {
    pub side: Side,
    pub bit_index_iterator: BitIndexIterator,
}

impl GroupPositionIterator {
    pub fn new(side: Side) -> Self {
        Self {
            side,
            bit_index_iterator: BitIndexIterator::default(),
        }
    }

    pub fn current_position(&self) -> Option<GroupPosition> {
        self.bit_index_iterator
            .current_index
            .map(|bit_index| GroupPosition::from_bit_index(self.side, bit_index))
    }

    pub fn set_current_position(&mut self, position: Option<GroupPosition>) {
        let bit_index = position.map(|position| position.bit_index(self.side));
        self.bit_index_iterator.set_current_index(bit_index);
    }

    pub fn peek(&self) -> Option<GroupPosition> {
        self.bit_index_iterator
            .peek()
            .map(|bit_index| GroupPosition::from_bit_index(self.side, bit_index))
    }
}

impl Iterator for GroupPositionIterator {
    type Item = GroupPosition;

    fn next(&mut self) -> Option<Self::Item> {
        self.bit_index_iterator
            .next()
            .map(|bit_index| GroupPosition::from_bit_index(self.side, bit_index))
    }
}

#[cfg(test)]
pub mod tests {
    use crate::state::{InnerIndex, RestingOrderIndex};

    use super::*;

    #[test]
    fn test_full_range_traversal() {
        test_full_range_for_side(Side::Ask);
        test_full_range_for_side(Side::Bid);
    }

    #[test]
    fn test_set_current_position() {
        set_current_position(Side::Ask);
        set_current_position(Side::Bid);
    }

    fn test_full_range_for_side(side: Side) {
        let mut iterator = GroupPositionIterator::new(side);

        assert_eq!(iterator.current_position(), None);

        for bit_index in 0..=255 {
            let expected_group_position = Some(GroupPosition::from_bit_index(side, bit_index));
            assert_eq!(iterator.peek(), expected_group_position);
            assert_eq!(iterator.next(), expected_group_position);
            assert_eq!(iterator.current_position(), expected_group_position);
        }

        assert_eq!(iterator.peek(), None);
        assert_eq!(iterator.next(), None);
        assert_eq!(
            iterator.current_position(),
            Some(GroupPosition::from_bit_index(side, 255))
        );
    }

    fn set_current_position(side: Side) {
        let mut iterator = GroupPositionIterator::new(side);

        // Last resting order index
        let group_position_0 = Some(GroupPosition {
            inner_index: InnerIndex::new(0),
            resting_order_index: RestingOrderIndex::new(7),
        });
        iterator.set_current_position(group_position_0);
        assert_eq!(iterator.current_position(), group_position_0);

        // Some inner index
        let group_position_1 = Some(GroupPosition {
            inner_index: InnerIndex::new(2),
            resting_order_index: RestingOrderIndex::new(0),
        });
        iterator.set_current_position(group_position_1);
        assert_eq!(iterator.current_position(), group_position_1);
    }
}
