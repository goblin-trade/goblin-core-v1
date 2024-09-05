use crate::state::{InnerIndex, RestingOrderIndex, Side};

use super::BitmapGroup;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GroupPosition {
    pub inner_index: InnerIndex,
    pub resting_order_index: RestingOrderIndex,
}

impl GroupPosition {
    pub fn count(&self, side: Side) -> u8 {
        let GroupPosition {
            inner_index,
            resting_order_index,
        } = self;

        match side {
            Side::Bid => {
                (31 - inner_index.as_usize() as u8) * 8 + (resting_order_index.as_u8() + 1)
            }

            Side::Ask => (inner_index.as_usize() as u8) * 8 + (resting_order_index.as_u8() + 1),
        }
    }
}

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

/// Iterator to find coordinates of active bits in a bitmap group
pub struct BitmapIterator<'a> {
    /// The bitmap group to search
    bitmap_group: &'a BitmapGroup,

    /// Iterator to obtain bitmap group coordinates
    group_position_iterator: GroupPositionIterator,
}

impl<'a> BitmapIterator<'a> {
    pub fn new(bitmap_group: &'a BitmapGroup, side: Side, size: u8) -> Self {
        BitmapIterator {
            bitmap_group,
            group_position_iterator: GroupPositionIterator::new(side, size),
        }
    }
}

impl<'a> Iterator for BitmapIterator<'a> {
    type Item = GroupPosition;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(GroupPosition {
            inner_index,
            resting_order_index,
        }) = self.group_position_iterator.next()
        {
            #[cfg(test)]
            println!(
                "inner_index {:?}, resting_order_index {:?}",
                inner_index, resting_order_index
            );
            let bitmap = self.bitmap_group.get_bitmap(&inner_index);

            if bitmap.order_present(resting_order_index) {
                return Some(GroupPosition {
                    inner_index,
                    resting_order_index,
                });
            }
        }

        None
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

    #[test]
    fn test_bitmap_group_iterator_same_bitmap_no_last_position() {
        let mut bitmap_group = BitmapGroup::default();
        bitmap_group.inner[0] = 0b10000011;

        let side = Side::Ask;
        let count = 0;

        let mut iterator = BitmapIterator::new(&bitmap_group, side, count);
        assert_eq!(
            iterator.next().unwrap(),
            GroupPosition {
                inner_index: InnerIndex::new(0),
                resting_order_index: RestingOrderIndex::new(0)
            }
        );
        assert_eq!(
            iterator.next().unwrap(),
            GroupPosition {
                inner_index: InnerIndex::new(0),
                resting_order_index: RestingOrderIndex::new(1)
            }
        );
        assert_eq!(
            iterator.next().unwrap(),
            GroupPosition {
                inner_index: InnerIndex::new(0),
                resting_order_index: RestingOrderIndex::new(7)
            }
        );
        assert!(iterator.next().is_none());
    }

    #[test]
    fn test_bitmap_group_iterator_same_bitmap_with_last_position() {
        let mut bitmap_group = BitmapGroup::default();
        bitmap_group.inner[0] = 0b10000011;

        let side = Side::Ask;
        let last_position = GroupPosition {
            inner_index: InnerIndex::ZERO,
            resting_order_index: RestingOrderIndex::ZERO,
        };
        let count = last_position.count(side);
        assert_eq!(count, 1);

        let mut iterator = BitmapIterator::new(&bitmap_group, side, count);
        assert_eq!(
            iterator.next().unwrap(),
            GroupPosition {
                inner_index: InnerIndex::new(0),
                resting_order_index: RestingOrderIndex::new(1)
            }
        );
        assert_eq!(
            iterator.next().unwrap(),
            GroupPosition {
                inner_index: InnerIndex::new(0),
                resting_order_index: RestingOrderIndex::new(7)
            }
        );
        assert!(iterator.next().is_none());
    }

    #[test]
    fn test_bitmap_group_iterator_consecutive_bitmaps_no_last_position() {
        let mut bitmap_group = BitmapGroup::default();
        bitmap_group.inner[0] = 0b00000001;
        bitmap_group.inner[1] = 0b10000000;

        let side = Side::Ask;
        let count = 0;

        let mut iterator = BitmapIterator::new(&bitmap_group, side, count);
        assert_eq!(
            iterator.next().unwrap(),
            GroupPosition {
                inner_index: InnerIndex::new(0),
                resting_order_index: RestingOrderIndex::new(0)
            }
        );
        assert_eq!(
            iterator.next().unwrap(),
            GroupPosition {
                inner_index: InnerIndex::new(1),
                resting_order_index: RestingOrderIndex::new(7)
            }
        );
        assert!(iterator.next().is_none());
    }

    #[test]
    fn test_bitmap_group_iterator_consecutive_bitmaps_with_last_position() {
        let mut bitmap_group = BitmapGroup::default();
        bitmap_group.inner[0] = 0b00000001;
        bitmap_group.inner[1] = 0b10000000;

        let side = Side::Ask;
        let last_position = GroupPosition {
            inner_index: InnerIndex::ZERO,
            resting_order_index: RestingOrderIndex::ZERO,
        };
        let count = last_position.count(side);

        let mut iterator = BitmapIterator::new(&bitmap_group, side, count);
        assert_eq!(
            iterator.next().unwrap(),
            GroupPosition {
                inner_index: InnerIndex::new(1),
                resting_order_index: RestingOrderIndex::new(7)
            }
        );
        assert!(iterator.next().is_none());
    }

    #[test]
    fn test_bitmap_group_iterator_non_consecutive_bitmaps_no_last_position() {
        let mut bitmap_group = BitmapGroup::default();
        bitmap_group.inner[0] = 0b00000001;
        bitmap_group.inner[10] = 0b00000001;

        let side = Side::Ask;
        let count = 0;

        let mut iterator = BitmapIterator::new(&bitmap_group, side, count);
        assert_eq!(
            iterator.next().unwrap(),
            GroupPosition {
                inner_index: InnerIndex::new(0),
                resting_order_index: RestingOrderIndex::new(0)
            }
        );
        assert_eq!(
            iterator.next().unwrap(),
            GroupPosition {
                inner_index: InnerIndex::new(10),
                resting_order_index: RestingOrderIndex::new(0)
            }
        );
        assert!(iterator.next().is_none());
    }

    #[test]
    fn test_bitmap_group_iterator_non_consecutive_bitmaps_with_last_position() {
        let mut bitmap_group = BitmapGroup::default();
        bitmap_group.inner[0] = 0b00000001;
        bitmap_group.inner[1] = 0b00000010;
        bitmap_group.inner[10] = 0b10000000;

        let side = Side::Ask;
        let last_position = GroupPosition {
            inner_index: InnerIndex::ONE,
            resting_order_index: RestingOrderIndex::ONE,
        };
        let count = last_position.count(side);
        assert_eq!(count, 10);

        let mut iterator = BitmapIterator::new(&bitmap_group, side, count);
        assert_eq!(
            iterator.next().unwrap(),
            GroupPosition {
                inner_index: InnerIndex::new(10),
                resting_order_index: RestingOrderIndex::MAX
            }
        );
        assert!(iterator.next().is_none());
    }

    #[test]
    fn test_bitmap_group_iterator_bids_same_bitmap_no_last_position() {
        let mut bitmap_group = BitmapGroup::default();
        bitmap_group.inner[31] = 0b10000011; // InnerIndex::MAX

        let side = Side::Bid;
        let count = 0;

        let mut iterator = BitmapIterator::new(&bitmap_group, side, count);
        assert_eq!(
            iterator.next().unwrap(),
            GroupPosition {
                inner_index: InnerIndex::new(31),
                resting_order_index: RestingOrderIndex::new(0)
            }
        );
        assert_eq!(
            iterator.next().unwrap(),
            GroupPosition {
                inner_index: InnerIndex::new(31),
                resting_order_index: RestingOrderIndex::new(1)
            }
        );
        assert_eq!(
            iterator.next().unwrap(),
            GroupPosition {
                inner_index: InnerIndex::new(31),
                resting_order_index: RestingOrderIndex::new(7)
            }
        );
        assert!(iterator.next().is_none());
    }

    #[test]
    fn test_bitmap_group_iterator_bids_same_bitmap_with_last_position() {
        let mut bitmap_group = BitmapGroup::default();
        bitmap_group.inner[31] = 0b10000011; // InnerIndex::MAX

        let side = Side::Bid;
        let last_position = GroupPosition {
            inner_index: InnerIndex::new(31),
            resting_order_index: RestingOrderIndex::new(0),
        };
        let count = last_position.count(side);
        assert_eq!(count, 1);

        let mut iterator = BitmapIterator::new(&bitmap_group, side, count);
        assert_eq!(
            iterator.next().unwrap(),
            GroupPosition {
                inner_index: InnerIndex::new(31),
                resting_order_index: RestingOrderIndex::new(1)
            }
        );
        assert_eq!(
            iterator.next().unwrap(),
            GroupPosition {
                inner_index: InnerIndex::new(31),
                resting_order_index: RestingOrderIndex::new(7)
            }
        );
        assert!(iterator.next().is_none());
    }

    #[test]
    fn test_bitmap_group_iterator_bids_consecutive_bitmaps_no_last_position() {
        let mut bitmap_group = BitmapGroup::default();
        bitmap_group.inner[31] = 0b00000001; // InnerIndex::MAX
        bitmap_group.inner[30] = 0b10000000;

        let side = Side::Bid;
        let count = 0;

        let mut iterator = BitmapIterator::new(&bitmap_group, side, count);
        assert_eq!(
            iterator.next().unwrap(),
            GroupPosition {
                inner_index: InnerIndex::new(31),
                resting_order_index: RestingOrderIndex::new(0)
            }
        );
        assert_eq!(
            iterator.next().unwrap(),
            GroupPosition {
                inner_index: InnerIndex::new(30),
                resting_order_index: RestingOrderIndex::new(7)
            }
        );
        assert!(iterator.next().is_none());
    }

    #[test]
    fn test_bitmap_group_iterator_bids_consecutive_bitmaps_with_last_position() {
        let mut bitmap_group = BitmapGroup::default();
        bitmap_group.inner[31] = 0b00000001; // InnerIndex::MAX
        bitmap_group.inner[30] = 0b10000000;

        let side = Side::Bid;
        let last_position = GroupPosition {
            inner_index: InnerIndex::new(31),
            resting_order_index: RestingOrderIndex::new(0),
        };
        let count = last_position.count(side);
        assert_eq!(count, 1);

        let mut iterator = BitmapIterator::new(&bitmap_group, side, count);
        assert_eq!(
            iterator.next().unwrap(),
            GroupPosition {
                inner_index: InnerIndex::new(30),
                resting_order_index: RestingOrderIndex::new(7)
            }
        );
        assert!(iterator.next().is_none());
    }

    #[test]
    fn test_bitmap_group_iterator_bids_non_consecutive_bitmaps_no_last_position() {
        let mut bitmap_group = BitmapGroup::default();
        bitmap_group.inner[31] = 0b00000001; // InnerIndex::MAX
        bitmap_group.inner[21] = 0b00000001;

        let side = Side::Bid;
        let count = 0;

        let mut iterator = BitmapIterator::new(&bitmap_group, side, count);
        assert_eq!(
            iterator.next().unwrap(),
            GroupPosition {
                inner_index: InnerIndex::new(31),
                resting_order_index: RestingOrderIndex::new(0)
            }
        );
        assert_eq!(
            iterator.next().unwrap(),
            GroupPosition {
                inner_index: InnerIndex::new(21),
                resting_order_index: RestingOrderIndex::new(0)
            }
        );
        assert!(iterator.next().is_none());
    }

    #[test]
    fn test_bitmap_group_iterator_bids_non_consecutive_bitmaps_with_last_position() {
        let mut bitmap_group = BitmapGroup::default();
        bitmap_group.inner[31] = 0b00000001; // InnerIndex::MAX
        bitmap_group.inner[30] = 0b00000010;
        bitmap_group.inner[21] = 0b10000000;

        let side = Side::Bid;
        let last_position = GroupPosition {
            inner_index: InnerIndex::new(30),
            resting_order_index: RestingOrderIndex::new(1),
        };
        let count = last_position.count(side);
        assert_eq!(count, 10);

        let mut iterator = BitmapIterator::new(&bitmap_group, side, count);
        assert_eq!(
            iterator.next().unwrap(),
            GroupPosition {
                inner_index: InnerIndex::new(21),
                resting_order_index: RestingOrderIndex::MAX
            }
        );
        assert!(iterator.next().is_none());
    }
}
