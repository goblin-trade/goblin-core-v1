use crate::state::{
    bitmap_group::BitmapGroup, iterator::position::group_position_iterator::GroupPositionIterator,
    order::group_position::GroupPosition, Side,
};

/// Iterator to find coordinates of active bits in a bitmap group
pub struct ActiveGroupPositionIterator {
    /// The bitmap group to search
    pub bitmap_group: BitmapGroup,

    /// Iterator to obtain bitmap group coordinates
    pub group_position_iterator: GroupPositionIterator,
}

impl ActiveGroupPositionIterator {
    pub fn new(bitmap_group: BitmapGroup, side: Side, count: u8) -> Self {
        ActiveGroupPositionIterator {
            bitmap_group,
            group_position_iterator: GroupPositionIterator::new(side, count),
        }
    }

    pub fn new_default_for_side(side: Side) -> Self {
        ActiveGroupPositionIterator {
            bitmap_group: BitmapGroup::default(),
            group_position_iterator: GroupPositionIterator::new(side, 0),
        }
    }

    pub fn new_with_starting_position(
        bitmap_group: BitmapGroup,
        side: Side,
        starting_position_inclusive: GroupPosition,
    ) -> Self {
        let count = starting_position_inclusive.count_inclusive(side);
        ActiveGroupPositionIterator::new(bitmap_group, side, count)
    }
}

impl Iterator for ActiveGroupPositionIterator {
    type Item = GroupPosition;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(GroupPosition {
            inner_index,
            resting_order_index,
        }) = self.group_position_iterator.next()
        {
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
    use crate::state::{InnerIndex, RestingOrderIndex};

    use super::*;

    // BitmapGroupIterator tests start here

    #[test]
    fn test_bitmap_group_iterator_same_bitmap_no_last_position() {
        let mut bitmap_group = BitmapGroup::default();
        bitmap_group.inner[0] = 0b10000011;

        let side = Side::Ask;
        let count = 0;

        let mut iterator = ActiveGroupPositionIterator::new(bitmap_group, side, count);
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
        let count = last_position.count_exclusive(side);
        assert_eq!(count, 1);

        let mut iterator = ActiveGroupPositionIterator::new(bitmap_group, side, count);
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

        let mut iterator = ActiveGroupPositionIterator::new(bitmap_group, side, count);
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
        let last_position_to_skip = GroupPosition {
            inner_index: InnerIndex::ZERO,
            resting_order_index: RestingOrderIndex::ZERO,
        };
        let count = last_position_to_skip.count_exclusive(side);

        let mut iterator = ActiveGroupPositionIterator::new(bitmap_group, side, count);
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

        let mut iterator = ActiveGroupPositionIterator::new(bitmap_group, side, count);
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
        let last_position_to_skip = GroupPosition {
            inner_index: InnerIndex::ONE,
            resting_order_index: RestingOrderIndex::ONE,
        };
        let count = last_position_to_skip.count_exclusive(side);
        assert_eq!(count, 10);

        let mut iterator = ActiveGroupPositionIterator::new(bitmap_group, side, count);
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

        let mut iterator = ActiveGroupPositionIterator::new(bitmap_group, side, count);
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
        let last_position_to_skip = GroupPosition {
            inner_index: InnerIndex::new(31),
            resting_order_index: RestingOrderIndex::new(0),
        };
        let count = last_position_to_skip.count_exclusive(side);
        assert_eq!(count, 1);

        let mut iterator = ActiveGroupPositionIterator::new(bitmap_group, side, count);
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

        let mut iterator = ActiveGroupPositionIterator::new(bitmap_group, side, count);
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
        let last_position_to_skip = GroupPosition {
            inner_index: InnerIndex::new(31),
            resting_order_index: RestingOrderIndex::new(0),
        };
        let count = last_position_to_skip.count_exclusive(side);
        assert_eq!(count, 1);

        let mut iterator = ActiveGroupPositionIterator::new(bitmap_group, side, count);
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

        let mut iterator = ActiveGroupPositionIterator::new(bitmap_group, side, count);
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
        let last_position_to_skip = GroupPosition {
            inner_index: InnerIndex::new(30),
            resting_order_index: RestingOrderIndex::new(1),
        };
        let count = last_position_to_skip.count_exclusive(side);
        assert_eq!(count, 10);

        let mut iterator = ActiveGroupPositionIterator::new(bitmap_group, side, count);
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
