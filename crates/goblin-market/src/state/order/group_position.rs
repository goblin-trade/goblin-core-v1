use crate::state::{InnerIndex, RestingOrderIndex, Side};

use super::order_id::OrderId;

/// Position of a bit within a bitmap group denoted as (inner_index, resting_order_index)
///
/// Group position has an alternate  form (side, bit_index), found by bit_index() function
///
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

    pub fn initial_for_side(side: Side) -> Self {
        GroupPosition {
            inner_index: match side {
                Side::Bid => InnerIndex::MAX,
                Side::Ask => InnerIndex::MIN,
            },
            // Resting order index is 0 for starting positions of both bids and asks
            resting_order_index: RestingOrderIndex::MIN,
        }
    }

    /// Convert a group position from (inner_index, resting_order_index) to its bit position
    /// in the range [0, 255] for the given side
    ///
    /// # Examples
    ///
    /// * (Ask, inner_index = 0, resting_order_index = 0): bit index 0, i.e.
    /// first item to be traversed
    ///
    /// * (Bid, inner_index = 0, resting_order_index = 0): bit index 248, i.e. first item
    /// on the last row. We traverse from top to bottom for bids.
    ///
    /// * (Ask, inner_index = 0, resting_order_index = 7): bit index 7, i.e.
    /// last item on the first row
    ///
    /// * (Bid, inner_index = 0, resting_order_index = 7): bit index 255, i.e. the last item
    ///
    pub fn bit_index(&self, side: Side) -> u8 {
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

    /// Convert the group index to group position for the given side
    ///
    /// # Arguments
    ///
    /// * `side`
    /// * `index_inclusive` - The index of the current position
    ///
    /// # Example
    ///
    /// * (Ask, index = 0) =>
    pub fn from_bit_index(side: Side, bit_index: u8) -> Self {
        let bit_index = match side {
            Side::Bid => 255 - bit_index, // Top to bottom for bids
            Side::Ask => bit_index,
        };

        let inner_index = InnerIndex::new(bit_index as usize / 8);

        let resting_order_index = RestingOrderIndex::new(match side {
            // We always move row-wise, so adjust resting_order_index to move from
            // left to right for bids
            Side::Bid => 7 - (bit_index % 8),
            Side::Ask => bit_index % 8,
        });

        GroupPosition {
            inner_index,
            resting_order_index,
        }
    }

    // TODO clear. from_index_inclusive() is being used instead
    //
    // /// Calculate the starting position for GroupPositionIterator
    // /// u8::MAX equals 255. A bitmap group has 256 bits
    // ///
    // /// (0, 0) yields count 1. (31, 6) yields count 254. The last position (32, 7)
    // /// causes count to overflow.
    // ///
    // /// Solution 1- use u16
    // /// Solution 2- externally ensure that 'last' values are not passed
    // ///
    // /// What is count?
    // /// - If count is 0, start from index 0. Index 0 is generated when count is None.
    // /// - If count is 1, start from index 1. Index 0 is skipped.
    // /// - If count is 255, start from index (31, 7), i.e. the last position. (31, 6) is skipped.
    // ///
    // /// The last group position (31, 7) should not be used to calculate count, since
    // /// in this case we must begin lookup in the next bitmap group with count 0.
    // pub fn count_exclusive(&self, side: Side) -> u8 {
    //     debug_assert!(
    //         side == Side::Bid && *self != GroupPosition::MIN
    //             || side == Side::Ask && *self != GroupPosition::MAX,
    //         "GroupPosition::MIN count is invalid for bids and GroupPosition::MAX count is invalid for asks"
    //     );

    //     self.count_inclusive(side) + 1

    //     // let GroupPosition {
    //     //     inner_index,
    //     //     resting_order_index,
    //     // } = self;

    //     // // Resting orders always begin from left to right so the latter part is the same
    //     // match side {
    //     //     Side::Bid => {
    //     //         (31 - inner_index.as_usize() as u8) * 8 + (resting_order_index.as_u8() + 1)
    //     //     }

    //     //     Side::Ask => (inner_index.as_usize() as u8) * 8 + (resting_order_index.as_u8() + 1),
    //     // }
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
    fn test_index_inclusive() {
        let position_0 = GroupPosition::MIN;
        assert_eq!(position_0.bit_index(Side::Ask), 0);
        assert_eq!(position_0.bit_index(Side::Bid), 248);

        let position_1 = GroupPosition {
            inner_index: InnerIndex::ZERO,
            resting_order_index: RestingOrderIndex::MAX,
        };
        assert_eq!(position_1.bit_index(Side::Ask), 7);
        assert_eq!(position_1.bit_index(Side::Bid), 255);

        let position_2 = GroupPosition {
            inner_index: InnerIndex::MAX,
            resting_order_index: RestingOrderIndex::ZERO,
        };
        assert_eq!(position_2.bit_index(Side::Ask), 248);
        assert_eq!(position_2.bit_index(Side::Bid), 0);

        let position_3 = GroupPosition::MAX;
        assert_eq!(position_3.bit_index(Side::Ask), 255);
        assert_eq!(position_3.bit_index(Side::Bid), 7);
    }

    #[test]
    fn get_for_zero_index() {
        let index_inclusive = 255;
        let position_bid = GroupPosition::from_bit_index(Side::Bid, index_inclusive);
        let position_ask = GroupPosition::from_bit_index(Side::Ask, index_inclusive);

        println!(
            "position_bid {:?}, position_ask {:?}",
            position_bid, position_ask
        );
    }

    #[test]
    fn test_range_for_bids() {
        let side = Side::Bid;
        for bit_index in 0..255 {
            let position = GroupPosition::from_bit_index(side, bit_index);
            println!("{:?}", position);
        }
    }

    // #[test]
    // fn test_count_for_asks() {
    //     let side = Side::Ask;

    //     let position_0 = GroupPosition::MIN;
    //     assert_eq!(position_0.count_exclusive(side), 1);

    //     let position_1 = GroupPosition {
    //         inner_index: InnerIndex::MAX,
    //         resting_order_index: RestingOrderIndex::new(6),
    //     };
    //     assert_eq!(position_1.count_exclusive(side), 255);
    // }

    // #[test]
    // fn test_count_for_bids() {
    //     let side = Side::Bid;

    //     let position_0 = GroupPosition {
    //         inner_index: InnerIndex::MAX,
    //         resting_order_index: RestingOrderIndex::ZERO,
    //     };
    //     assert_eq!(position_0.count_exclusive(side), 1);

    //     let position_1 = GroupPosition {
    //         inner_index: InnerIndex::MAX,
    //         resting_order_index: RestingOrderIndex::MAX,
    //     };
    //     assert_eq!(position_1.count_exclusive(side), 8);

    //     let position_2 = GroupPosition {
    //         inner_index: InnerIndex::MIN,
    //         resting_order_index: RestingOrderIndex::new(6),
    //     };
    //     assert_eq!(position_2.count_exclusive(side), 255);
    // }
}
