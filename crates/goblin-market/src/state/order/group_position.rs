use crate::state::{InnerIndex, RestingOrderIndex, Side};

use super::order_id::OrderId;

// Position of a bit within a bitmap gorup
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GroupPosition {
    pub inner_index: InnerIndex,
    pub resting_order_index: RestingOrderIndex,
}

impl GroupPosition {
    pub const MIN: GroupPosition = GroupPosition {
        inner_index: InnerIndex::MIN,
        resting_order_index: RestingOrderIndex::MIN,
    };

    pub const MAX: GroupPosition = GroupPosition {
        inner_index: InnerIndex::MAX,
        resting_order_index: RestingOrderIndex::MAX,
    };

    /// Calculate the starting position for GroupPositionIterator
    /// u8::MAX equals 255. A bitmap group has 256 bits
    ///
    /// (0, 0) yields count 1. (31, 6) yields count 254. The last position (32, 7)
    /// causes count to overflow.
    ///
    /// Solution 1- use u16
    /// Solution 2- externally ensure that 'last' values are not passed
    ///
    /// What is count?
    /// - If count is 0, start from index 0. Index 0 is generated when count is None.
    /// - If count is 1, start from index 1. Index 0 is skipped.
    /// - If count is 255, start from index (31, 7), i.e. the last position. (31, 6) is skipped.
    ///
    /// The last group position (31, 7) should not be used to calculate count, since
    /// in this case we must begin lookup in the next bitmap group with count 0.
    pub fn count_exclusive(&self, side: Side) -> u8 {
        debug_assert!(
            side == Side::Bid && *self != GroupPosition::MIN
                || side == Side::Ask && *self != GroupPosition::MAX,
            "GroupPosition::MIN count is invalid for bids and GroupPosition::MAX count is invalid for asks"
        );

        self.count_inclusive(side) + 1

        // let GroupPosition {
        //     inner_index,
        //     resting_order_index,
        // } = self;

        // // Resting orders always begin from left to right so the latter part is the same
        // match side {
        //     Side::Bid => {
        //         (31 - inner_index.as_usize() as u8) * 8 + (resting_order_index.as_u8() + 1)
        //     }

        //     Side::Ask => (inner_index.as_usize() as u8) * 8 + (resting_order_index.as_u8() + 1),
        // }
    }

    pub fn count_inclusive(&self, side: Side) -> u8 {
        let GroupPosition {
            inner_index,
            resting_order_index,
        } = self;

        (match side {
            Side::Ask => *inner_index,
            Side::Bid => InnerIndex::MAX - *inner_index,
        })
        .as_usize() as u8
            * 8
            + resting_order_index.as_u8()
    }

    // pub fn count_inclusive(&self, side: Side) -> u8 {
    //     let coordinates = self.coordinates();
    //     match side {
    //         Side::Bid => 255 - coordinates,
    //         Side::Ask => coordinates,
    //     }
    // }
}

impl From<&OrderId> for GroupPosition {
    fn from(value: &OrderId) -> Self {
        GroupPosition {
            inner_index: value.price_in_ticks.inner_index(),
            resting_order_index: value.resting_order_index,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::state::{InnerIndex, RestingOrderIndex, Side};

    use super::GroupPosition;

    #[test]
    fn test_count_inclusive() {
        let position_0 = GroupPosition::MIN;
        assert_eq!(position_0.count_inclusive(Side::Ask), 0);
        assert_eq!(position_0.count_inclusive(Side::Bid), 248);

        let position_1 = GroupPosition {
            inner_index: InnerIndex::ZERO,
            resting_order_index: RestingOrderIndex::MAX,
        };
        assert_eq!(position_1.count_inclusive(Side::Ask), 7);
        assert_eq!(position_1.count_inclusive(Side::Bid), 255);

        let position_2 = GroupPosition {
            inner_index: InnerIndex::MAX,
            resting_order_index: RestingOrderIndex::ZERO,
        };
        assert_eq!(position_2.count_inclusive(Side::Ask), 248);
        assert_eq!(position_2.count_inclusive(Side::Bid), 0);

        let position_3 = GroupPosition::MAX;
        assert_eq!(position_3.count_inclusive(Side::Ask), 255);
        assert_eq!(position_3.count_inclusive(Side::Bid), 7);
    }

    #[test]
    fn test_count_for_asks() {
        let side = Side::Ask;

        let position_0 = GroupPosition::MIN;
        assert_eq!(position_0.count_exclusive(side), 1);

        let position_1 = GroupPosition {
            inner_index: InnerIndex::MAX,
            resting_order_index: RestingOrderIndex::new(6),
        };
        assert_eq!(position_1.count_exclusive(side), 255);
    }

    #[test]
    fn test_count_for_bids() {
        let side = Side::Bid;

        let position_0 = GroupPosition {
            inner_index: InnerIndex::MAX,
            resting_order_index: RestingOrderIndex::ZERO,
        };
        assert_eq!(position_0.count_exclusive(side), 1);

        let position_1 = GroupPosition {
            inner_index: InnerIndex::MAX,
            resting_order_index: RestingOrderIndex::MAX,
        };
        assert_eq!(position_1.count_exclusive(side), 8);

        let position_2 = GroupPosition {
            inner_index: InnerIndex::MIN,
            resting_order_index: RestingOrderIndex::new(6),
        };
        assert_eq!(position_2.count_exclusive(side), 255);
    }
}
