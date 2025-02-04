use crate::state::{InnerIndex, RestingOrderIndex, Side};

use super::order_id::OrderId;

/// Position of a bit within a bitmap group denoted as the tuple (inner_index, resting_order_index)
///
/// Group position has an alternate form (side, bit_index), found by bit_index() function
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GroupPosition {
    pub inner_index: InnerIndex,
    pub resting_order_index: RestingOrderIndex,
}

impl GroupPosition {
    /// The first group position for the given side.
    /// It always starts with resting order index 0 irrespective of side.
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

    /// Convert a group position to bit index for the given side
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

    /// Convert bit index and side to group position
    ///
    /// # Arguments
    ///
    /// * `side`
    /// * `bit_index`
    ///
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
}

// Convert OrderID to group position
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
    use super::*;

    #[test]
    fn test_group_positions_match_bit_indices() {
        group_positions_match_bit_indices(Side::Ask, 0..32);
        group_positions_match_bit_indices(Side::Bid, (0..32).rev());
    }

    fn group_positions_match_bit_indices(
        side: Side,
        inner_index_range: impl Iterator<Item = usize>,
    ) {
        let mut expected_bit_index = 0;
        for inner_index in inner_index_range {
            for resting_order_index in 0..8 {
                let position = GroupPosition {
                    inner_index: InnerIndex::new(inner_index),
                    resting_order_index: RestingOrderIndex::new(resting_order_index),
                };
                let bit_index = position.bit_index(side);
                assert_eq!(bit_index, expected_bit_index);
                let position_from_bit_index = GroupPosition::from_bit_index(side, bit_index);
                assert_eq!(position, position_from_bit_index);

                if expected_bit_index != 255 {
                    expected_bit_index += 1;
                }
            }
        }
    }
}
