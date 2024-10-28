use crate::state::{order::group_position::GroupPosition, Side};

/// Efficient iterator to loop through Group positions of a bitmap group.
///
/// In addition to iterations, this struct is used to paginate across
/// group positions in the order lookup remover.
#[derive(Debug)]
pub struct GroupPositionIterator {
    /// Side determines looping direction.
    /// - Bids: Top to bottom (descending)
    /// - Asks: Bottom to top (ascending)
    pub side: Side,

    /// Index of the next element to be returned, from 0 to 255 (inclusive)
    pub next_index: u8,

    /// Whether iteration is complete.
    /// Since u8 can store 256 states but we have an additional 257th state
    /// for completed iteration, use a boolean flag to track whether iteration
    /// is completed. When iteration is complete next_index = 255 and exhausted = true.
    pub exhausted: bool,
}

impl GroupPositionIterator {
    pub fn new(side: Side, index: u8) -> Self {
        GroupPositionIterator {
            side,
            next_index: index,
            exhausted: false,
        }
    }

    /// Returns the upcoming group position to be returned by next()
    fn peek(&self) -> Option<GroupPosition> {
        if self.exhausted {
            return None;
        }
        Some(GroupPosition::from_index_inclusive(
            self.side,
            self.next_index,
        ))
    }

    // Lookup remover utils

    /// Return the group position that was returned by .next() previously.
    /// Also acts as a getter for group position in lookup remover.
    pub fn peek_previous(&self) -> Option<GroupPosition> {
        self.previous_index()
            .map(|previous_index| GroupPosition::from_index_inclusive(self.side, previous_index))
    }

    /// Paginate the iterator to a given group position, setting it as
    /// the previous position.
    ///
    /// This is used by GroupPositionLookupRemover::find() to paginate
    /// to a position and check whether it is active. We use the previous and
    /// not next position for pagination because the sequential remover removes
    /// the previous position. The lookup remover invokes the sequential remover
    /// in a special case.
    ///
    /// # Arguments
    ///
    /// * `group_position`
    pub fn set_previous_position(&mut self, group_position: GroupPosition) {
        let previous_index = group_position.index_inclusive(self.side);
        self.set_previous_index(previous_index);
    }

    /// Index of the previous value that was returned by the iterator.
    ///
    /// The previous index is one position behind next_index.
    fn previous_index(&self) -> Option<u8> {
        if self.next_index == 0 {
            // next_index is zero, i.e. iterator is freshly initialized
            // and next() was never called. There is no previous position.
            None
        } else if self.exhausted {
            // If iterator is exhausted, the previous index was 255
            Some(255)
        } else {
            // General case- previous index is one position behind next index
            Some(self.next_index - 1)
        }
    }

    /// Paginates to the given previous index by updating inner variables
    fn set_previous_index(&mut self, previous_index: u8) {
        if previous_index == 255 {
            self.next_index = 255;
            self.exhausted = true;
        } else {
            self.next_index = previous_index + 1;
            self.exhausted = false;
        };
    }
}

impl Iterator for GroupPositionIterator {
    type Item = GroupPosition;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.peek();

        // increment if less than 255, else set exhausted to true
        if self.next_index == 255 {
            self.exhausted = true;
        } else {
            self.next_index += 1;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{InnerIndex, RestingOrderIndex};

    mod test_iteration {
        use super::*;

        #[test]
        fn test_get_indices_for_asks() {
            let side = Side::Ask;
            let next_index = 0;

            let mut iterator = GroupPositionIterator::new(side, next_index);
            assert!(iterator.peek_previous().is_none());

            for i in 0..=255 {
                assert_eq!(iterator.next_index, i);
                assert_eq!(iterator.exhausted, false);

                let expected_position = Some(GroupPosition::from_index_inclusive(side, i));

                let peeked_position = iterator.peek();
                let next_position = iterator.next();
                let peeked_previous_position = iterator.peek_previous();

                assert_eq!(peeked_position, expected_position);
                assert_eq!(next_position, expected_position);
                assert_eq!(peeked_previous_position, expected_position);
            }
            assert_eq!(iterator.next_index, 255);
            assert_eq!(iterator.exhausted, true);
            assert!(iterator.peek().is_none());
            assert!(iterator.next().is_none());
            assert_eq!(
                iterator.peek_previous(),
                Some(GroupPosition::from_index_inclusive(side, 255))
            );
        }

        #[test]
        fn test_get_indices_for_asks_with_index_10() {
            let side = Side::Ask;
            let next_index = 10;

            let mut iterator = GroupPositionIterator::new(side, next_index);
            assert_eq!(
                iterator.peek_previous().unwrap(),
                GroupPosition {
                    inner_index: InnerIndex::ONE,
                    resting_order_index: RestingOrderIndex::new(1)
                }
            );
            assert_eq!(
                iterator.peek().unwrap(),
                GroupPosition {
                    inner_index: InnerIndex::ONE,
                    resting_order_index: RestingOrderIndex::new(2)
                }
            );

            for i in 10..=255 {
                assert_eq!(iterator.next_index, i);
                assert_eq!(iterator.exhausted, false);

                let expected_position = Some(GroupPosition::from_index_inclusive(side, i));

                let peeked_position = iterator.peek();
                let next_position = iterator.next();
                let peeked_previous_position = iterator.peek_previous();

                assert_eq!(peeked_position, expected_position);
                assert_eq!(next_position, expected_position);
                assert_eq!(peeked_previous_position, expected_position);
            }
            assert_eq!(iterator.next_index, 255);
            assert_eq!(iterator.exhausted, true);
            assert!(iterator.peek().is_none());
            assert!(iterator.next().is_none());
            assert_eq!(
                iterator.peek_previous(),
                Some(GroupPosition::from_index_inclusive(side, 255))
            );
        }

        #[test]
        fn test_get_indices_for_bids() {
            let side = Side::Bid;
            let next_index = 0;

            let mut iterator = GroupPositionIterator::new(side, next_index);
            assert!(iterator.peek_previous().is_none());

            for i in 0..=255 {
                assert_eq!(iterator.next_index, i);
                assert_eq!(iterator.exhausted, false);

                let expected_position = Some(GroupPosition::from_index_inclusive(side, i));

                let peeked_position = iterator.peek();
                let next_position = iterator.next();
                let peeked_previous_position = iterator.peek_previous();

                assert_eq!(peeked_position, expected_position);
                assert_eq!(next_position, expected_position);
                assert_eq!(peeked_previous_position, expected_position);
            }
            assert_eq!(iterator.next_index, 255);
            assert_eq!(iterator.exhausted, true);
            assert!(iterator.peek().is_none());
            assert!(iterator.next().is_none());
            assert_eq!(
                iterator.peek_previous(),
                Some(GroupPosition::from_index_inclusive(side, 255))
            );
        }

        #[test]
        fn test_get_indices_for_bids_with_index_10() {
            let side = Side::Bid;
            let next_index = 10;

            let mut iterator = GroupPositionIterator::new(side, next_index);
            assert_eq!(
                iterator.peek_previous().unwrap(),
                GroupPosition {
                    inner_index: InnerIndex::new(30),
                    resting_order_index: RestingOrderIndex::new(1)
                }
            );
            assert_eq!(
                iterator.peek().unwrap(),
                GroupPosition {
                    inner_index: InnerIndex::new(30),
                    resting_order_index: RestingOrderIndex::new(2)
                }
            );

            for i in 10..=255 {
                assert_eq!(iterator.next_index, i);
                assert_eq!(iterator.exhausted, false);

                let expected_position = Some(GroupPosition::from_index_inclusive(side, i));

                let peeked_position = iterator.peek();
                let next_position = iterator.next();
                let peeked_previous_position = iterator.peek_previous();

                assert_eq!(peeked_position, expected_position);
                assert_eq!(next_position, expected_position);
                assert_eq!(peeked_previous_position, expected_position);
            }
            assert_eq!(iterator.next_index, 255);
            assert_eq!(iterator.exhausted, true);
            assert!(iterator.peek().is_none());
            assert!(iterator.next().is_none());
            assert_eq!(
                iterator.peek_previous(),
                Some(GroupPosition::from_index_inclusive(side, 255))
            );
        }
    }

    mod test_lookup {
        use super::*;

        #[test]
        fn test_set_previous_index() {
            let side = Side::Ask;
            let next_index = 0;
            let mut iterator = GroupPositionIterator::new(side, next_index);

            assert_eq!(iterator.previous_index(), None);

            for previous_index in 0..=254 {
                iterator.set_previous_index(previous_index);
                assert_eq!(iterator.previous_index().unwrap(), previous_index);
                assert_eq!(iterator.next_index, previous_index + 1);
                assert_eq!(iterator.exhausted, false);
            }

            iterator.set_previous_index(255);
            assert_eq!(iterator.previous_index().unwrap(), 255);
            assert_eq!(iterator.next_index, 255);
            assert_eq!(iterator.exhausted, true);
            assert_eq!(iterator.peek(), None);
        }

        #[test]
        fn test_set_previous_index_on_exhausted() {
            let side = Side::Ask;
            for previous_index in 0..=255 {
                let mut iterator = GroupPositionIterator {
                    side,
                    next_index: 255,
                    exhausted: true,
                };
                iterator.set_previous_index(previous_index);
                assert_eq!(iterator.previous_index().unwrap(), previous_index);
            }
        }
    }
}
