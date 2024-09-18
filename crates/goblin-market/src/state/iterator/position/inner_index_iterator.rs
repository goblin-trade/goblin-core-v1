use crate::state::{InnerIndex, MarketPrices, OuterIndex, Side};

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

    /// Stop iterator when `max_count` is reached
    pub max_count: usize,
}

impl InnerIndexIterator {
    pub fn new(side: Side) -> Self {
        InnerIndexIterator {
            side,
            count: 0,
            max_count: 32,
        }
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

        InnerIndexIterator {
            side,
            count,
            max_count: 32,
        }
    }

    pub fn new_between_market_prices(
        best_market_prices: &MarketPrices,
        outer_index: OuterIndex,
    ) -> Self {
        let MarketPrices {
            best_bid_price,
            best_ask_price,
        } = best_market_prices;

        let bid_inner_index = if best_bid_price.outer_index() == outer_index {
            Some(best_bid_price.inner_index())
        } else {
            None
        };
        let ask_inner_index = if best_ask_price.outer_index() == outer_index {
            Some(best_ask_price.inner_index())
        } else {
            None
        };

        InnerIndexIterator::new_between_inner_indices(bid_inner_index, ask_inner_index)
    }

    /// Iterates through inner indices lying between inner indices of best bid and
    /// best ask where both bounds are exclusive. The iteration direction is
    /// ascending, i.e. Ask like.
    ///
    /// Used to clear garbage values in a bitmap group so direction doesn't matter.
    /// If any of the best prices doesn't fall on the current outer index, it is passed as None.
    ///
    /// Externally ensure that `ask_inner_index` and `ask_inner_index` are not equal if Some,
    /// because these are exclusive bounds.
    ///
    /// # Arguments
    ///
    /// * `bid_inner_index` - Inner index of best bid if it falls on the current outer index,
    /// else None.
    /// * `ask_inner_index` - Inner index of best ask if it falls on the current outer index,
    /// else None.
    ///
    pub fn new_between_inner_indices(
        bid_inner_index: Option<InnerIndex>,
        ask_inner_index: Option<InnerIndex>,
    ) -> Self {
        debug_assert!(bid_inner_index.is_none() || (bid_inner_index != ask_inner_index));

        let side = Side::Ask;
        let count = bid_inner_index.map(|i| i.as_usize() + 1).unwrap_or(0);
        let max_count = ask_inner_index.map(|i| i.as_usize()).unwrap_or(32);

        InnerIndexIterator {
            side,
            count,
            max_count,
        }
    }
}

impl Iterator for InnerIndexIterator {
    type Item = InnerIndex;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count == self.max_count {
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

    mod new_with_starting_index {
        use super::*;

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

    mod new_between_inner_indices {
        use super::*;

        #[test]
        fn test_between_best_prices_both_none() {
            let bid_inner_index = None;
            let ask_inner_index = None;

            let mut iterator =
                InnerIndexIterator::new_between_inner_indices(bid_inner_index, ask_inner_index);

            for i in 0..=31 {
                assert_eq!(iterator.next().unwrap(), InnerIndex::new(i));
            }
            assert!(iterator.next().is_none());
        }

        #[test]
        fn test_between_best_prices_with_bid_inner_index() {
            let bid_inner_index = Some(InnerIndex::ZERO);
            let ask_inner_index = None;

            let mut iterator =
                InnerIndexIterator::new_between_inner_indices(bid_inner_index, ask_inner_index);

            for i in 1..=31 {
                assert_eq!(iterator.next().unwrap(), InnerIndex::new(i));
            }
            assert!(iterator.next().is_none());
        }

        #[test]
        fn test_between_best_prices_with_ask_inner_index() {
            let bid_inner_index = None;
            let ask_inner_index = Some(InnerIndex::MAX);

            let mut iterator =
                InnerIndexIterator::new_between_inner_indices(bid_inner_index, ask_inner_index);

            for i in 0..=30 {
                assert_eq!(iterator.next().unwrap(), InnerIndex::new(i));
            }
            assert!(iterator.next().is_none());
        }

        #[test]
        fn test_between_best_prices_with_both_indices() {
            let bid_inner_index = Some(InnerIndex::ZERO);
            let ask_inner_index = Some(InnerIndex::MAX);

            let mut iterator =
                InnerIndexIterator::new_between_inner_indices(bid_inner_index, ask_inner_index);

            for i in 1..=30 {
                assert_eq!(iterator.next().unwrap(), InnerIndex::new(i));
            }
            assert!(iterator.next().is_none());
        }

        #[test]
        fn test_between_best_prices_with_both_indices_and_no_gap() {
            let bid_inner_index = Some(InnerIndex::ZERO);
            let ask_inner_index = Some(InnerIndex::ONE);

            let mut iterator =
                InnerIndexIterator::new_between_inner_indices(bid_inner_index, ask_inner_index);
            assert!(iterator.next().is_none());
        }
    }
}
