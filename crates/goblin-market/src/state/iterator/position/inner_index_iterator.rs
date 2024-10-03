use crate::{
    quantities::Ticks,
    state::{InnerIndex, MarketPrices, OuterIndex, Side},
};

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
    /// TODO replace with u8
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
    pub fn new_with_starting_index(
        side: Side,
        starting_index_inclusive: Option<InnerIndex>,
    ) -> Self {
        let count = if let Some(start_index_inclusive) = starting_index_inclusive {
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

    /// Generates inner indices that hold garbage bits.
    ///
    /// The loop direction is always ascending, i.e. ask like.
    ///
    /// # Arguments
    ///
    /// * `outer_index` - Outer index of the bitmap group being looped through
    /// * `best_market_prices` - The best market prices BEFORE performing any add
    /// or remove operation
    ///
    pub fn new_for_garbage_bits(
        outer_index: OuterIndex,
        best_market_prices: &MarketPrices,
    ) -> Self {
        let MarketPrices {
            best_bid_price,
            best_ask_price,
        } = *best_market_prices;

        let group_start_price = Ticks::from_indices(outer_index, InnerIndex::ZERO);
        let group_end_price = Ticks::from_indices(outer_index, InnerIndex::MAX);

        let (count, max_count) =
            if best_bid_price > group_end_price || best_ask_price < group_start_price {
                (32, 32)
            } else {
                let start_price_inclusive = (best_bid_price + Ticks::ONE).max(group_start_price);
                let end_price_inclusive = (best_ask_price - Ticks::ONE).min(group_end_price);

                let count = start_price_inclusive.inner_index().as_usize();
                let max_count = end_price_inclusive.inner_index().as_usize() + 1;

                (count, max_count)
            };

        InnerIndexIterator {
            side: Side::Ask,
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

    mod new_between_market_prices_v4 {
        use crate::quantities::Ticks;

        use super::*;

        #[test]
        fn test_both_best_prices_on_same_outer_index() {
            let outer_index_0 = OuterIndex::new(0);
            let outer_index_1 = OuterIndex::new(1);
            let outer_index_2 = OuterIndex::new(2);

            let best_bid_price = Ticks::from_indices(outer_index_1, InnerIndex::ZERO);
            let best_ask_price = Ticks::from_indices(outer_index_1, InnerIndex::MAX);

            let best_market_prices = MarketPrices {
                best_bid_price,
                best_ask_price,
            };
            // Outer index less than best_bid_outer_index
            let mut iterator_0 =
                InnerIndexIterator::new_for_garbage_bits(outer_index_0, &best_market_prices);
            assert!(iterator_0.next().is_none());

            // Outer index equal to best_bid_outer_index and best_ask_outer_index
            let mut iterator_1 =
                InnerIndexIterator::new_for_garbage_bits(outer_index_1, &best_market_prices);
            for i in 1..=30 {
                assert_eq!(iterator_1.next().unwrap().as_usize(), i);
            }
            assert!(iterator_1.next().is_none());

            // Outer index more than best_ask_outer_index
            let mut iterator_2 =
                InnerIndexIterator::new_for_garbage_bits(outer_index_2, &best_market_prices);
            assert!(iterator_2.next().is_none());
        }

        #[test]
        fn test_best_prices_on_consecutive_outer_indices() {
            let outer_index_0 = OuterIndex::new(0);
            let outer_index_1 = OuterIndex::new(1);
            let outer_index_2 = OuterIndex::new(2);
            let outer_index_3 = OuterIndex::new(3);

            let best_bid_price = Ticks::from_indices(outer_index_1, InnerIndex::new(1));
            let best_ask_price = Ticks::from_indices(outer_index_2, InnerIndex::new(2));

            let best_market_prices = MarketPrices {
                best_bid_price,
                best_ask_price,
            };
            // Outer index less than best_bid_outer_index
            let mut iterator_0 =
                InnerIndexIterator::new_for_garbage_bits(outer_index_0, &best_market_prices);
            assert!(iterator_0.next().is_none());

            // Outer index equal to best_bid_outer_index but less than best_ask_outer_index
            let mut iterator_1 =
                InnerIndexIterator::new_for_garbage_bits(outer_index_1, &best_market_prices);
            for i in 2..=31 {
                assert_eq!(iterator_1.next().unwrap().as_usize(), i);
            }
            assert!(iterator_1.next().is_none());

            // Outer index more than best_bid_outer_index and equal to best_ask_outer_index
            let mut iterator_2 =
                InnerIndexIterator::new_for_garbage_bits(outer_index_2, &best_market_prices);
            for i in 0..=1 {
                assert_eq!(iterator_2.next().unwrap().as_usize(), i);
            }
            assert!(iterator_2.next().is_none());

            // Outer index more than best_ask_outer_index
            let mut iterator_3 =
                InnerIndexIterator::new_for_garbage_bits(outer_index_3, &best_market_prices);
            assert!(iterator_3.next().is_none());
        }

        #[test]
        fn test_best_prices_on_non_consecutive_outer_indices() {
            let outer_index_0 = OuterIndex::new(0);
            let outer_index_1 = OuterIndex::new(1);
            let outer_index_2 = OuterIndex::new(2);
            let outer_index_3 = OuterIndex::new(3);
            let outer_index_4 = OuterIndex::new(4);

            let best_bid_price = Ticks::from_indices(outer_index_1, InnerIndex::new(1));
            let best_ask_price = Ticks::from_indices(outer_index_3, InnerIndex::new(2));

            let best_market_prices = MarketPrices {
                best_bid_price,
                best_ask_price,
            };
            // Outer index less than best_bid_outer_index
            let mut iterator_0 =
                InnerIndexIterator::new_for_garbage_bits(outer_index_0, &best_market_prices);
            assert!(iterator_0.next().is_none());

            // Outer index equal to best_bid_outer_index but less than best_ask_outer_index
            let mut iterator_1 =
                InnerIndexIterator::new_for_garbage_bits(outer_index_1, &best_market_prices);
            for i in 2..=31 {
                assert_eq!(iterator_1.next().unwrap().as_usize(), i);
            }
            assert!(iterator_1.next().is_none());

            // Outer index between best_bid_outer_index and best_ask_outer_index
            let mut iterator_2 =
                InnerIndexIterator::new_for_garbage_bits(outer_index_2, &best_market_prices);
            // TODO error here- gives None
            for i in 0..=31 {
                assert_eq!(iterator_2.next().unwrap().as_usize(), i);
            }
            assert!(iterator_2.next().is_none());

            // Outer index more than best_bid_outer_index and equal to best_ask_outer_index
            let mut iterator_3 =
                InnerIndexIterator::new_for_garbage_bits(outer_index_3, &best_market_prices);
            for i in 0..=1 {
                assert_eq!(iterator_3.next().unwrap().as_usize(), i);
            }
            assert!(iterator_3.next().is_none());

            // Outer index more than best_ask_outer_index
            let mut iterator_4 =
                InnerIndexIterator::new_for_garbage_bits(outer_index_4, &best_market_prices);
            assert!(iterator_4.next().is_none());
        }
    }
}
