use crate::state::{order::group_position::GroupPosition, Side};

/// Efficient iterator to loop through Group positions (inner index, resting order index)
/// of a bitmap group.
pub struct GroupPositionIterator {
    /// Side determines looping direction.
    /// - Bids: Top to bottom (descending)
    /// - Asks: Bottom to top (ascending)
    pub side: Side,

    /// Position of the element from 0 to 255 (inclusive)
    pub index: u8,

    /// Whether iteration is complete.
    /// Special property of iterators- we need a flag to know when to stop.
    /// Using the value itself is not sufficient.
    finished: bool,
}

impl GroupPositionIterator {
    pub fn new(side: Side, index: u8) -> Self {
        GroupPositionIterator {
            side,
            index,
            finished: false,
        }
    }

    /// Returns the last group position that was returned by the iterator
    pub fn last_group_position(&self) -> Option<GroupPosition> {
        if !self.finished && self.index == 0 {
            return None;
        }

        // finished, index = 0 will wrap to 255 which gives the last value correctly
        let last_index = self.index.wrapping_sub(1);
        Some(GroupPosition::from_index_inclusive(self.side, last_index))
    }

    /// Returns the next group position that will be returned by the iterator
    pub fn next_group_position(&self) -> Option<GroupPosition> {
        if self.finished {
            return None;
        }
        Some(GroupPosition::from_index_inclusive(self.side, self.index))
    }
}
impl Iterator for GroupPositionIterator {
    type Item = GroupPosition;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.next_group_position();

        self.index = self.index.wrapping_add(1);
        self.finished = self.index == 0;

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{InnerIndex, RestingOrderIndex};

    mod test_for_starting_index {

        use super::*;

        #[test]
        fn test_get_indices_for_asks() {
            let side = Side::Ask;
            let index = 0;

            let mut iterator = GroupPositionIterator::new(side, index);
            assert!(iterator.last_group_position().is_none());

            for i in 0..=255 {
                let bit_index = i;

                let expected_inner_index = InnerIndex::new(bit_index as usize / 8);
                let expected_resting_order_index = RestingOrderIndex::new(bit_index % 8);

                let GroupPosition {
                    inner_index,
                    resting_order_index,
                } = iterator.next().unwrap();

                let current_position = iterator.last_group_position().unwrap();
                assert_eq!(inner_index, current_position.inner_index);
                assert_eq!(resting_order_index, current_position.resting_order_index);

                println!(
                    "inner_index {:?}, resting_order_index {:?}",
                    inner_index, resting_order_index
                );

                assert_eq!(inner_index, expected_inner_index);
                assert_eq!(resting_order_index, expected_resting_order_index);

                if i == 255 {
                    assert_eq!(iterator.index, 0);
                } else {
                    assert_eq!(iterator.index, i + 1);
                }
            }
            assert!(iterator.next().is_none());
        }

        #[test]
        fn test_get_indices_for_asks_with_index_10() {
            let side = Side::Ask;
            let index = 10;

            let mut iterator = GroupPositionIterator::new(side, index);
            assert_eq!(
                iterator.last_group_position().unwrap(),
                GroupPosition {
                    inner_index: InnerIndex::ONE,
                    resting_order_index: RestingOrderIndex::new(1)
                }
            );

            for i in 10..=255 {
                let bit_index = i;

                let expected_inner_index = InnerIndex::new(bit_index as usize / 8);
                let expected_resting_order_index = RestingOrderIndex::new(bit_index % 8);

                let GroupPosition {
                    inner_index,
                    resting_order_index,
                } = iterator.next().unwrap();

                let current_position = iterator.last_group_position().unwrap();
                assert_eq!(inner_index, current_position.inner_index);
                assert_eq!(resting_order_index, current_position.resting_order_index);

                println!(
                    "inner_index {:?}, resting_order_index {:?}",
                    inner_index, resting_order_index
                );

                assert_eq!(inner_index, expected_inner_index);
                assert_eq!(resting_order_index, expected_resting_order_index);

                if i == 255 {
                    assert_eq!(iterator.index, 0);
                } else {
                    assert_eq!(iterator.index, i + 1);
                }
            }
            assert!(iterator.next().is_none());
        }

        #[test]
        fn test_get_indices_for_bids() {
            let side = Side::Bid;
            let index = 0;

            let mut iterator = GroupPositionIterator::new(side, index);
            assert!(iterator.last_group_position().is_none());

            for i in 0..=255 {
                let bit_index = 255 - i;

                let expected_inner_index = InnerIndex::new(bit_index as usize / 8);
                let expected_resting_order_index = RestingOrderIndex::new(match side {
                    Side::Bid => 7 - (bit_index % 8),
                    Side::Ask => bit_index % 8,
                });

                let GroupPosition {
                    inner_index,
                    resting_order_index,
                } = iterator.next().unwrap();

                let current_position = iterator.last_group_position().unwrap();
                assert_eq!(inner_index, current_position.inner_index);
                assert_eq!(resting_order_index, current_position.resting_order_index);

                println!(
                    "inner_index {:?}, resting_order_index {:?}",
                    inner_index, resting_order_index
                );

                assert_eq!(inner_index, expected_inner_index);
                assert_eq!(resting_order_index, expected_resting_order_index);

                if i == 255 {
                    assert_eq!(iterator.index, 0);
                } else {
                    assert_eq!(iterator.index, i + 1);
                }
            }
            assert!(iterator.next().is_none());
        }

        #[test]
        fn test_get_indices_for_bids_with_index_10() {
            let side = Side::Bid;
            let index = 10;

            let mut iterator = GroupPositionIterator::new(side, index);
            assert_eq!(
                iterator.last_group_position().unwrap(),
                GroupPosition {
                    inner_index: InnerIndex::new(30),
                    resting_order_index: RestingOrderIndex::new(1)
                }
            );

            for i in 10..=255 {
                let bit_index = 255 - i;

                let expected_inner_index = InnerIndex::new(bit_index as usize / 8);
                let expected_resting_order_index = RestingOrderIndex::new(match side {
                    Side::Bid => 7 - (bit_index % 8),
                    Side::Ask => bit_index % 8,
                });

                let GroupPosition {
                    inner_index,
                    resting_order_index,
                } = iterator.next().unwrap();

                let current_position = iterator.last_group_position().unwrap();
                assert_eq!(inner_index, current_position.inner_index);
                assert_eq!(resting_order_index, current_position.resting_order_index);

                println!(
                    "inner_index {:?}, resting_order_index {:?}",
                    inner_index, resting_order_index
                );

                assert_eq!(inner_index, expected_inner_index);
                assert_eq!(resting_order_index, expected_resting_order_index);

                if i == 255 {
                    assert_eq!(iterator.index, 0);
                } else {
                    assert_eq!(iterator.index, i + 1);
                }
            }
            assert!(iterator.next().is_none());
        }
    }
}
