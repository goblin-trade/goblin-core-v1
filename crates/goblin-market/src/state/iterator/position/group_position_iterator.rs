use crate::state::{order::group_position::GroupPosition, InnerIndex, RestingOrderIndex, Side};

/// Efficient iterator to loop through coordinates (inner index, resting order index)
/// of a bitmap group.
///
///
pub struct GroupPositionIterator {
    /// Side determines looping direction.
    /// - Bids: Top to bottom (descending)
    /// - Asks: Bottom to top (ascending)
    pub side: Side,

    /// Number of elements traversed
    pub count: u8,

    /// Whether iteration is complete.
    /// Special property of iterators- we need a flag to know when to stop.
    /// Using the value itself is not sufficient.
    finished: bool,
}

impl GroupPositionIterator {
    pub fn new(side: Side, count: u8) -> Self {
        GroupPositionIterator {
            side,
            count,
            finished: false,
        }
    }
}
impl Iterator for GroupPositionIterator {
    type Item = GroupPosition;

    fn next(&mut self) -> Option<Self::Item> {
        if self.finished {
            return None;
        }

        let bit_index = match self.side {
            Side::Bid => 255 - self.count,
            Side::Ask => self.count,
        };

        let inner_index = InnerIndex::new(bit_index as usize / 8);

        let resting_order_index = RestingOrderIndex::new(match self.side {
            Side::Bid => 7 - (bit_index % 8),
            Side::Ask => bit_index % 8,
        });

        let result = Some(GroupPosition {
            inner_index,
            resting_order_index,
        });

        self.count = self.count.wrapping_add(1);
        self.finished = self.count == 0;

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_indices_for_asks() {
        let side = Side::Ask;
        let count = 0;

        let mut iterator = GroupPositionIterator::new(side, count);

        for i in 0..=255 {
            let bit_index = match side {
                Side::Bid => 255 - 1 - i,
                Side::Ask => i,
            };

            let expected_inner_index = InnerIndex::new(bit_index as usize / 8);
            let expected_resting_order_index = RestingOrderIndex::new(bit_index % 8);

            let GroupPosition {
                inner_index,
                resting_order_index,
            } = iterator.next().unwrap();

            println!(
                "inner_index {:?}, resting_order_index {:?}",
                inner_index, resting_order_index
            );

            assert_eq!(inner_index, expected_inner_index);
            assert_eq!(resting_order_index, expected_resting_order_index);

            if i == 255 {
                assert_eq!(iterator.count, 0);
            } else {
                assert_eq!(iterator.count, i + 1);
            }
        }
    }

    #[test]
    fn test_get_indices_for_asks_with_count_10() {
        let side = Side::Ask;
        let count = 10;

        let mut iterator = GroupPositionIterator::new(side, count);

        for i in 10..=255 {
            let bit_index = match side {
                Side::Bid => 255 - 1 - i,
                Side::Ask => i,
            };

            let expected_inner_index = InnerIndex::new(bit_index as usize / 8);
            let expected_resting_order_index = RestingOrderIndex::new(bit_index % 8);

            let GroupPosition {
                inner_index,
                resting_order_index,
            } = iterator.next().unwrap();

            println!(
                "inner_index {:?}, resting_order_index {:?}",
                inner_index, resting_order_index
            );

            assert_eq!(inner_index, expected_inner_index);
            assert_eq!(resting_order_index, expected_resting_order_index);

            if i == 255 {
                assert_eq!(iterator.count, 0);
            } else {
                assert_eq!(iterator.count, i + 1);
            }
        }
    }

    #[test]
    fn test_get_indices_for_bids() {
        let side = Side::Bid;
        let count = 0;

        let mut iterator = GroupPositionIterator::new(side, count);

        for i in 0..=255 {
            let bit_index = match side {
                Side::Bid => 255 - i,
                Side::Ask => i,
            };

            let expected_inner_index = InnerIndex::new(bit_index as usize / 8);
            let expected_resting_order_index = RestingOrderIndex::new(match side {
                Side::Bid => 7 - (bit_index % 8),
                Side::Ask => bit_index % 8,
            });

            let GroupPosition {
                inner_index,
                resting_order_index,
            } = iterator.next().unwrap();

            println!(
                "inner_index {:?}, resting_order_index {:?}",
                inner_index, resting_order_index
            );

            assert_eq!(inner_index, expected_inner_index);
            assert_eq!(resting_order_index, expected_resting_order_index);

            if i == 255 {
                assert_eq!(iterator.count, 0);
            } else {
                assert_eq!(iterator.count, i + 1);
            }
        }
    }

    #[test]
    fn test_get_indices_for_bids_with_count_10() {
        let side = Side::Bid;
        let count = 10;

        let mut iterator = GroupPositionIterator::new(side, count);

        for i in 10..=255 {
            let bit_index = match side {
                Side::Bid => 255 - i,
                Side::Ask => i,
            };

            let expected_inner_index = InnerIndex::new(bit_index as usize / 8);
            let expected_resting_order_index = RestingOrderIndex::new(match side {
                Side::Bid => 7 - (bit_index % 8),
                Side::Ask => bit_index % 8,
            });

            let GroupPosition {
                inner_index,
                resting_order_index,
            } = iterator.next().unwrap();

            println!(
                "inner_index {:?}, resting_order_index {:?}",
                inner_index, resting_order_index
            );

            assert_eq!(inner_index, expected_inner_index);
            assert_eq!(resting_order_index, expected_resting_order_index);

            if i == 255 {
                assert_eq!(iterator.count, 0);
            } else {
                assert_eq!(iterator.count, i + 1);
            }
        }
    }
}
