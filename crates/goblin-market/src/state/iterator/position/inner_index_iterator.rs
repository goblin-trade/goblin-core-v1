use crate::{
    quantities::Ticks,
    state::{InnerIndex, MarketPrices, OuterIndex, Side, TickIndices},
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

    // TODO fix- subcases when max prices don't lie on outer index
    // - If outer index is 'between' the prices, clear its bits. This is needed for insertions.
    // - If outer index is behind one of the prices then there is nothing to clear
    pub fn new_between_market_prices_v2(
        best_market_prices: &MarketPrices,
        outer_index: OuterIndex,
    ) -> Self {
        let MarketPrices {
            best_bid_price,
            best_ask_price,
        } = best_market_prices;

        let TickIndices {
            outer_index: bid_outer_index,
            inner_index: bid_inner_index,
        } = best_bid_price.to_indices();

        let TickIndices {
            outer_index: ask_outer_index,
            inner_index: ask_inner_index,
        } = best_ask_price.to_indices();

        let (count, max_count) = match (
            bid_outer_index == outer_index,
            ask_outer_index == outer_index,
        ) {
            (true, true) => (bid_inner_index.as_usize() + 1, ask_inner_index.as_usize()),
            (true, false) => (bid_inner_index.as_usize() + 1, 32),
            (false, true) => (0, ask_inner_index.as_usize()),
            (false, false) => (0, 0),
        };

        InnerIndexIterator {
            side: Side::Ask,
            count,
            max_count,
        }
    }

    pub fn new_between_market_prices_v3(
        best_market_prices: &MarketPrices,
        outer_index: OuterIndex,
    ) -> Self {
        let MarketPrices {
            best_bid_price,
            best_ask_price,
        } = best_market_prices;

        let TickIndices {
            outer_index: bid_outer_index,
            inner_index: bid_inner_index,
        } = best_bid_price.to_indices();

        let TickIndices {
            outer_index: ask_outer_index,
            inner_index: ask_inner_index,
        } = best_ask_price.to_indices();

        let count = if outer_index == bid_outer_index {
            bid_inner_index.as_usize() + 1
        } else {
            0
        };

        let max_count = if outer_index < bid_outer_index {
            0
        } else if outer_index == ask_outer_index {
            ask_inner_index.as_usize()
        } else {
            32
        };

        InnerIndexIterator {
            side: Side::Ask,
            count,
            max_count,
        }
    }

    pub fn new_between_market_prices_v4(
        best_market_prices: &MarketPrices,
        outer_index: OuterIndex,
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
        debug_assert!(
            (bid_inner_index.is_some()
                && ask_inner_index.is_some()
                && ask_inner_index.unwrap() > bid_inner_index.unwrap())
                || bid_inner_index.is_none()
                || ask_inner_index.is_none()
        );

        // cases
        // 1. Both present- (bid_inner_index, ask_inner_index)
        // 2. bid_inner_index is None- [0, ask_inner_index)
        // 3. ask_inner_index is None- (bid_inner_index, 31]
        // 4. both None- (). Pass count and max_count as 0
        let side = Side::Ask;

        let (count, max_count) = match (bid_inner_index, ask_inner_index) {
            (Some(bid_inner_index), Some(ask_inner_index)) => {
                (bid_inner_index.as_usize() + 1, ask_inner_index.as_usize())
            }
            (Some(bid_inner_index), None) => (bid_inner_index.as_usize() + 1, 32),
            (None, Some(ask_inner_index)) => (0, ask_inner_index.as_usize()),
            (None, None) => (0, 0),
        };

        // let count = bid_inner_index.map(|i| i.as_usize() + 1).unwrap_or(0);
        // let max_count = ask_inner_index.map(|i| i.as_usize()).unwrap_or(32);

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

    // mod new_between_inner_indices {
    //     use super::*;

    //     #[test]
    //     fn test_between_best_prices_with_both_indices() {
    //         let bid_inner_index = Some(InnerIndex::ZERO);
    //         let ask_inner_index = Some(InnerIndex::MAX);

    //         let mut iterator =
    //             InnerIndexIterator::new_between_inner_indices(bid_inner_index, ask_inner_index);

    //         for i in 1..=30 {
    //             assert_eq!(iterator.next().unwrap(), InnerIndex::new(i));
    //         }
    //         assert!(iterator.next().is_none());
    //     }

    //     #[test]
    //     fn test_between_best_prices_with_both_indices_and_no_gap() {
    //         let bid_inner_index = Some(InnerIndex::ZERO);
    //         let ask_inner_index = Some(InnerIndex::ONE);

    //         let mut iterator =
    //             InnerIndexIterator::new_between_inner_indices(bid_inner_index, ask_inner_index);
    //         assert!(iterator.next().is_none());
    //     }

    //     #[test]
    //     fn test_between_best_prices_with_bid_inner_index() {
    //         let bid_inner_index = Some(InnerIndex::ZERO);
    //         let ask_inner_index = None;

    //         let mut iterator =
    //             InnerIndexIterator::new_between_inner_indices(bid_inner_index, ask_inner_index);

    //         for i in 1..=31 {
    //             assert_eq!(iterator.next().unwrap(), InnerIndex::new(i));
    //         }
    //         assert!(iterator.next().is_none());
    //     }

    //     #[test]
    //     fn test_between_best_prices_with_ask_inner_index() {
    //         let bid_inner_index = None;
    //         let ask_inner_index = Some(InnerIndex::MAX);

    //         let mut iterator =
    //             InnerIndexIterator::new_between_inner_indices(bid_inner_index, ask_inner_index);

    //         for i in 0..=30 {
    //             assert_eq!(iterator.next().unwrap(), InnerIndex::new(i));
    //         }
    //         assert!(iterator.next().is_none());
    //     }

    //     #[test]
    //     fn test_between_best_prices_both_none() {
    //         let bid_inner_index = None;
    //         let ask_inner_index = None;

    //         let mut iterator =
    //             InnerIndexIterator::new_between_inner_indices(bid_inner_index, ask_inner_index);

    //         assert!(iterator.next().is_none());
    //     }
    // }

    // mod new_between_market_prices {
    //     use crate::quantities::Ticks;

    //     use super::*;

    //     #[test]
    //     fn test_both_best_prices_on_outer_index() {
    //         let outer_index = OuterIndex::new(2);

    //         let best_bid_price = Ticks::from_indices(outer_index, InnerIndex::ZERO);
    //         let best_ask_price = Ticks::from_indices(outer_index, InnerIndex::MAX);

    //         let best_market_prices = MarketPrices {
    //             best_bid_price,
    //             best_ask_price,
    //         };
    //         let mut iterator =
    //             InnerIndexIterator::new_between_market_prices_v2(&best_market_prices, outer_index);

    //         for i in 1..=30 {
    //             assert_eq!(iterator.next().unwrap().as_usize(), i);
    //         }
    //         assert!(iterator.next().is_none());
    //     }

    //     #[test]
    //     fn test_both_best_prices_on_outer_index_no_gap() {
    //         let outer_index = OuterIndex::new(2);

    //         let best_bid_price = Ticks::from_indices(outer_index, InnerIndex::ZERO);
    //         let best_ask_price = Ticks::from_indices(outer_index, InnerIndex::ONE);

    //         let best_market_prices = MarketPrices {
    //             best_bid_price,
    //             best_ask_price,
    //         };
    //         let mut iterator =
    //             InnerIndexIterator::new_between_market_prices_v2(&best_market_prices, outer_index);

    //         assert!(iterator.next().is_none());
    //     }

    //     #[test]
    //     fn test_best_bid_price_is_not_on_outer_index() {
    //         let outer_index = OuterIndex::new(2);
    //         let outer_index_bid = OuterIndex::new(1);

    //         let best_bid_price = Ticks::from_indices(outer_index_bid, InnerIndex::ZERO);
    //         let best_ask_price = Ticks::from_indices(outer_index, InnerIndex::ONE);

    //         let best_market_prices = MarketPrices {
    //             best_bid_price,
    //             best_ask_price,
    //         };
    //         let mut iterator =
    //             InnerIndexIterator::new_between_market_prices_v2(&best_market_prices, outer_index);

    //         for i in 0..=30 {
    //             assert_eq!(iterator.next().unwrap().as_usize(), i);
    //         }
    //         assert!(iterator.next().is_none());
    //     }

    //     #[test]
    //     fn test_best_bid_price_is_not_on_outer_index_partial_range() {
    //         let outer_index = OuterIndex::new(2);
    //         let outer_index_bid = OuterIndex::new(1);

    //         let best_bid_price = Ticks::from_indices(outer_index_bid, InnerIndex::ZERO);
    //         let best_ask_price = Ticks::from_indices(outer_index, InnerIndex::ONE);

    //         let best_market_prices = MarketPrices {
    //             best_bid_price,
    //             best_ask_price,
    //         };
    //         let mut iterator =
    //             InnerIndexIterator::new_between_market_prices_v2(&best_market_prices, outer_index);

    //         assert_eq!(iterator.next().unwrap(), InnerIndex::ZERO);
    //         assert!(iterator.next().is_none());
    //     }

    //     #[test]
    //     fn test_best_prices_are_not_on_current_group() {
    //         let outer_index = OuterIndex::new(2);
    //         let outer_index_best_prices = OuterIndex::new(1);

    //         let best_bid_price = Ticks::from_indices(outer_index_best_prices, InnerIndex::ZERO);
    //         let best_ask_price = Ticks::from_indices(outer_index_best_prices, InnerIndex::ONE);

    //         let best_market_prices = MarketPrices {
    //             best_bid_price,
    //             best_ask_price,
    //         };
    //         let mut iterator =
    //             InnerIndexIterator::new_between_market_prices(&best_market_prices, outer_index);

    //         loop {
    //             let next_index = iterator.next();

    //             if next_index.is_none() {
    //                 break;
    //             }
    //             println!("Next index {:?}", next_index);
    //         }
    //     }
    // }

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
            let mut iterator_0 = InnerIndexIterator::new_between_market_prices_v4(
                &best_market_prices,
                outer_index_0,
            );
            println!(
                "count {}, max count {}",
                iterator_0.count, iterator_0.max_count
            );
            assert!(iterator_0.next().is_none());

            // Outer index equal to best_bid_outer_index and best_ask_outer_index
            let mut iterator_1 = InnerIndexIterator::new_between_market_prices_v4(
                &best_market_prices,
                outer_index_1,
            );
            for i in 1..=30 {
                assert_eq!(iterator_1.next().unwrap().as_usize(), i);
            }
            assert!(iterator_1.next().is_none());

            // Outer index more than best_ask_outer_index
            let mut iterator_2 = InnerIndexIterator::new_between_market_prices_v4(
                &best_market_prices,
                outer_index_2,
            );
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
            let mut iterator_0 = InnerIndexIterator::new_between_market_prices_v4(
                &best_market_prices,
                outer_index_0,
            );
            assert!(iterator_0.next().is_none());

            // Outer index equal to best_bid_outer_index but less than best_ask_outer_index
            let mut iterator_1 = InnerIndexIterator::new_between_market_prices_v4(
                &best_market_prices,
                outer_index_1,
            );
            for i in 2..=31 {
                assert_eq!(iterator_1.next().unwrap().as_usize(), i);
            }
            assert!(iterator_1.next().is_none());

            // Outer index more than best_bid_outer_index and equal to best_ask_outer_index
            let mut iterator_2 = InnerIndexIterator::new_between_market_prices_v4(
                &best_market_prices,
                outer_index_2,
            );
            for i in 0..=1 {
                assert_eq!(iterator_2.next().unwrap().as_usize(), i);
            }
            assert!(iterator_2.next().is_none());

            // Outer index more than best_ask_outer_index
            let mut iterator_3 = InnerIndexIterator::new_between_market_prices_v4(
                &best_market_prices,
                outer_index_3,
            );
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
            let mut iterator_0 = InnerIndexIterator::new_between_market_prices_v4(
                &best_market_prices,
                outer_index_0,
            );
            assert!(iterator_0.next().is_none());

            // Outer index equal to best_bid_outer_index but less than best_ask_outer_index
            let mut iterator_1 = InnerIndexIterator::new_between_market_prices_v4(
                &best_market_prices,
                outer_index_1,
            );
            for i in 2..=31 {
                assert_eq!(iterator_1.next().unwrap().as_usize(), i);
            }
            assert!(iterator_1.next().is_none());

            // Outer index between best_bid_outer_index and best_ask_outer_index
            let mut iterator_2 = InnerIndexIterator::new_between_market_prices_v4(
                &best_market_prices,
                outer_index_2,
            );
            // TODO error here- gives None
            for i in 0..=31 {
                assert_eq!(iterator_2.next().unwrap().as_usize(), i);
            }
            assert!(iterator_2.next().is_none());

            // Outer index more than best_bid_outer_index and equal to best_ask_outer_index
            let mut iterator_3 = InnerIndexIterator::new_between_market_prices_v4(
                &best_market_prices,
                outer_index_3,
            );
            for i in 0..=1 {
                assert_eq!(iterator_3.next().unwrap().as_usize(), i);
            }
            assert!(iterator_3.next().is_none());

            // Outer index more than best_ask_outer_index
            let mut iterator_4 = InnerIndexIterator::new_between_market_prices_v4(
                &best_market_prices,
                outer_index_4,
            );
            assert!(iterator_4.next().is_none());
        }
    }
}
