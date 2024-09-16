use crate::state::{InnerIndex, Side};

/// Iterates through consecutive values of InnerIndex, i.e.
/// 0 to 31 for Asks and 31 to 0 for bids (inclusive). The traversal
/// direction is away from centre of the book.
///
/// Optionally provides a way to iterate beginning from a given inner index.
pub struct InnerIndexIterator {
    /// Side determines looping direction.
    /// - Bids: Top to bottom (descending)
    /// - Asks: Bottom to top (ascending)
    pub side: Side,

    /// Number of outer indices traversed
    pub count: usize,
}

impl InnerIndexIterator {
    pub fn new(side: Side) -> Self {
        InnerIndexIterator { side, count: 0 }
    }

    /// Begin iteration from a starting position (inclusive)
    pub fn new_with_starting_index(side: Side, starting_index: Option<InnerIndex>) -> Self {
        let count = if let Some(start_index_inclusive) = starting_index {
            match side {
                Side::Bid => 31 - start_index_inclusive.as_usize(),
                Side::Ask => start_index_inclusive.as_usize(),
            }
        } else {
            0
        };

        InnerIndexIterator { side, count }
    }
}

impl Iterator for InnerIndexIterator {
    type Item = InnerIndex;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count == 32 {
            return None;
        }

        let next = Some(InnerIndex::new(match self.side {
            Side::Bid => 31 - self.count,
            Side::Ask => self.count,
        }));

        self.count += 1;
        next
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{InnerIndex, Side};

    #[test]
    fn test_ask_iterator_full_iteration() {
        // Start from 0 for Ask and iterate upwards
        let mut iterator = InnerIndexIterator::new(Side::Ask);

        for expected in 0..=31 {
            assert_eq!(iterator.next(), Some(InnerIndex::new(expected)));
        }

        // Ensure the iterator returns None after completing all values
        assert_eq!(iterator.next(), None);
    }

    #[test]
    fn test_bid_iterator_full_iteration() {
        // Start from 31 for Bid and iterate downwards
        let mut iterator = InnerIndexIterator::new(Side::Bid);

        for expected in (0..=31).rev() {
            assert_eq!(iterator.next(), Some(InnerIndex::new(expected)));
        }

        // Ensure the iterator returns None after completing all values
        assert_eq!(iterator.next(), None);
    }

    #[test]
    fn test_ask_with_start_index_0() {
        let start_index = Some(InnerIndex::ZERO);
        let mut iterator = InnerIndexIterator::new_with_starting_index(Side::Ask, start_index);

        for expected in 0..=31 {
            assert_eq!(iterator.next(), Some(InnerIndex::new(expected)));
        }
        assert_eq!(iterator.next(), None);
    }

    #[test]
    fn test_bid_with_start_index_max() {
        let start_index = Some(InnerIndex::MAX);
        let mut iterator = InnerIndexIterator::new_with_starting_index(Side::Bid, start_index);

        for expected in (0..=31).rev() {
            assert_eq!(iterator.next(), Some(InnerIndex::new(expected)));
        }
        assert_eq!(iterator.next(), None);
    }

    #[test]
    fn test_ask_with_start_index_max() {
        let start_index = Some(InnerIndex::MAX);
        let mut iterator = InnerIndexIterator::new_with_starting_index(Side::Ask, start_index);

        assert_eq!(iterator.next(), Some(InnerIndex::MAX));
        assert_eq!(iterator.next(), None);
    }

    #[test]
    fn test_bid_with_start_index_zero() {
        let start_index = Some(InnerIndex::ZERO);
        let mut iterator = InnerIndexIterator::new_with_starting_index(Side::Bid, start_index);

        assert_eq!(iterator.next(), Some(InnerIndex::ZERO));
        assert_eq!(iterator.next(), None);
    }

    #[test]
    fn test_ask_with_start_in_middle() {
        let start_index = Some(InnerIndex::new(15));
        let mut iterator = InnerIndexIterator::new_with_starting_index(Side::Ask, start_index);

        for expected in 15..=31 {
            assert_eq!(iterator.next(), Some(InnerIndex::new(expected)));
        }
        assert_eq!(iterator.next(), None);
    }

    #[test]
    fn test_bid_with_start_in_middle() {
        let start_index = Some(InnerIndex::new(15));
        let mut iterator = InnerIndexIterator::new_with_starting_index(Side::Bid, start_index);

        for expected in (0..=15).rev() {
            assert_eq!(iterator.next(), Some(InnerIndex::new(expected)));
        }
        assert_eq!(iterator.next(), None);
    }
}
