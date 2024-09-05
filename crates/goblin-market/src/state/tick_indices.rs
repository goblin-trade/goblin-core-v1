use core::ops::Add;
use core::ops::AddAssign;
use core::ops::Sub;
use core::ops::SubAssign;

use crate::state::Side;
use crate::state::BITMAPS_PER_GROUP;
use crate::{
    quantities::{Ticks, WrapperU64},
    state::{SlotKey, BITMAP_GROUP_SEED},
};

/// To read orders at a tick we need two to derive variables. The `group_key` gives
/// the bitmap group in the tick's bitmap belongs. The `bitmap_key` gives the location
/// of the tick.
///
/// Active ticks are tracked using two indices. The bitmap group for the tick is first
/// read using `group_key`.
///
/// Wait- resting order is read directly using the tick.
///

/// Orders at a tick are read with two indices- the outer index and inner index.
/// The outer index points to a slot having data of 32 ticks. The inner index gives
/// the bitmap for the tick
pub struct TickIndices {
    pub outer_index: OuterIndex,
    pub inner_index: InnerIndex,
}

impl Ticks {
    pub fn outer_index(&self) -> OuterIndex {
        // Since max size of Ticks is 2^21 - 1, division by 2^5 ensures that it fits in u16
        OuterIndex::new((self.as_u64() / BITMAPS_PER_GROUP) as u16)
    }

    pub fn inner_index(&self) -> InnerIndex {
        InnerIndex::new((self.as_u64() % BITMAPS_PER_GROUP) as usize)
    }

    pub fn to_indices(&self) -> TickIndices {
        TickIndices {
            outer_index: self.outer_index(),
            inner_index: self.inner_index(),
        }
    }

    pub fn from_indices(outer_index: OuterIndex, inner_index: InnerIndex) -> Ticks {
        Ticks::new(outer_index.as_u16() as u64 * BITMAPS_PER_GROUP + inner_index.as_usize() as u64)
    }
}

/// Key to fetch a Bitmap group. A Bitmap consists of multiple Bitmaps
#[derive(Clone, Copy, PartialEq, PartialOrd, Debug)]
#[repr(transparent)]
pub struct OuterIndex {
    /// Index of bitmap group
    pub inner: u16,
}

impl SubAssign for OuterIndex {
    fn sub_assign(&mut self, other: OuterIndex) {
        self.inner -= other.inner;
    }
}

impl AddAssign for OuterIndex {
    fn add_assign(&mut self, other: OuterIndex) {
        self.inner += other.inner;
    }
}

impl OuterIndex {
    pub const ZERO: Self = OuterIndex { inner: 0 };
    pub const ONE: Self = OuterIndex { inner: 1 };
    pub const MAX: Self = OuterIndex { inner: u16::MAX };

    pub fn new(inner: u16) -> Self {
        OuterIndex { inner }
    }

    pub fn as_u16(&self) -> u16 {
        self.inner
    }

    pub fn is_closer_to_center(&self, side: Side, other: OuterIndex) -> bool {
        match side {
            Side::Bid => other > *self,
            Side::Ask => other < *self,
        }
    }
}

impl SlotKey for OuterIndex {
    fn get_key(&self) -> [u8; 32] {
        let mut key = [0u8; 32];

        key[0] = BITMAP_GROUP_SEED;
        key[1..3].copy_from_slice(&self.inner.to_be_bytes());

        key
    }
}

/// Key to fetch the bitmap within a bitmap group
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(transparent)]
pub struct InnerIndex {
    /// Relative position of the bitmap within the bitmap group
    inner: usize,
}

impl InnerIndex {
    pub const ZERO: InnerIndex = InnerIndex { inner: 0 };
    pub const MIN: InnerIndex = InnerIndex::ZERO;
    pub const ONE: InnerIndex = InnerIndex { inner: 1 };
    pub const MAX: InnerIndex = InnerIndex { inner: 31 };

    // TODO replace assert
    pub fn new(inner: usize) -> Self {
        assert!(inner < BITMAPS_PER_GROUP as usize);
        InnerIndex { inner }
    }

    pub fn as_usize(&self) -> usize {
        self.inner
    }
}

impl Add for InnerIndex {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        InnerIndex {
            inner: self.inner.wrapping_add(other.inner),
        }
    }
}

impl Sub for InnerIndex {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        InnerIndex {
            inner: self.inner.wrapping_sub(other.inner),
        }
    }
}

impl AddAssign for InnerIndex {
    fn add_assign(&mut self, other: InnerIndex) {
        self.inner += other.inner;
    }
}

impl SubAssign for InnerIndex {
    fn sub_assign(&mut self, other: InnerIndex) {
        self.inner = self.inner.wrapping_sub(other.inner);
    }
}

/// Loop across inner indices in a bitmap group
/// TODO use InnerIndexIterator and RestingOrderIndexIterator in place of raw loops
pub struct InnerIndexIterator {
    /// Side determines looping direction.
    /// - Bids: Top to bottom (descending)
    /// - Asks: Bottom to top (ascending)
    side: Side,

    /// The last returned value, used as an index while traversal
    /// Begin looping one position ahead of `last_position`
    last_index: Option<InnerIndex>,
}

impl InnerIndexIterator {
    /// Initializes a new InnerIndexIterator
    ///
    /// # Arguments
    ///
    /// * `side` - Decides loop direction
    /// * `last_index` - The last position to exclude while looping
    ///
    pub fn new(side: Side, last_index: Option<InnerIndex>) -> Self {
        InnerIndexIterator { side, last_index }
    }
}

impl Iterator for InnerIndexIterator {
    type Item = InnerIndex;

    fn next(&mut self) -> Option<Self::Item> {
        match self.side {
            Side::Ask => {
                if let Some(last_position) = self.last_index {
                    if last_position == InnerIndex::MAX {
                        return None; // Reached the end
                    }
                    self.last_index = Some(last_position + InnerIndex::ONE);
                } else {
                    self.last_index = Some(InnerIndex::ZERO)
                }
            }
            Side::Bid => {
                if let Some(last_position) = self.last_index {
                    if last_position == InnerIndex::ZERO {
                        return None; // Reached the end
                    }
                    self.last_index = Some(last_position - InnerIndex::ONE);
                } else {
                    self.last_index = Some(InnerIndex::MAX)
                }
            }
        }
        self.last_index
    }
}

#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
#[repr(transparent)]
pub struct RestingOrderIndex {
    inner: u8,
}

impl RestingOrderIndex {
    pub const ZERO: RestingOrderIndex = RestingOrderIndex { inner: 0 };
    pub const MIN: RestingOrderIndex = RestingOrderIndex::ZERO;
    pub const ONE: RestingOrderIndex = RestingOrderIndex { inner: 1 };
    pub const MAX: RestingOrderIndex = RestingOrderIndex { inner: 7 };

    pub fn new(inner: u8) -> Self {
        assert!(inner <= RestingOrderIndex::MAX.inner);
        RestingOrderIndex { inner }
    }

    pub fn as_u8(&self) -> u8 {
        self.inner
    }
}

impl Add for RestingOrderIndex {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        RestingOrderIndex {
            inner: self.inner.wrapping_add(other.inner),
        }
    }
}

impl AddAssign for RestingOrderIndex {
    fn add_assign(&mut self, other: RestingOrderIndex) {
        self.inner += other.inner;
    }
}

/// Loop from the min (0) to max (7) value of RestingOrderIndex
pub struct RestingOrderIndexIterator {
    /// Begin lookup one position ahead of `last_index`.
    /// This acts as an index for lookups.
    last_index: Option<RestingOrderIndex>,
}

impl RestingOrderIndexIterator {
    /// Initializes a new RestingOrderIndexIterator
    ///
    /// # Arguments
    ///
    /// * `last_index` - The last position to exclude while looping
    ///
    pub fn new(last_index: Option<RestingOrderIndex>) -> Self {
        RestingOrderIndexIterator { last_index }
    }
}

impl Iterator for RestingOrderIndexIterator {
    type Item = RestingOrderIndex;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(last_index) = self.last_index {
            if last_index == RestingOrderIndex::MAX {
                return None; // Reached the end
            }
            self.last_index = Some(last_index + RestingOrderIndex::ONE);
        } else {
            self.last_index = Some(RestingOrderIndex::ZERO);
        }
        self.last_index
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inner_index_iterator_ask_start_none() {
        // Side = Ask, last_position = None
        let mut iter = InnerIndexIterator::new(Side::Ask, None);

        // Loop through values from 0 to 31
        for i in 0..=InnerIndex::MAX.as_usize() {
            assert_eq!(iter.next(), Some(InnerIndex::new(i)));
        }

        // After reaching MAX (31), next should return None
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_inner_index_iterator_ask_start_zero() {
        // Side = Ask, last_position = Some(InnerIndex::ZERO)
        let mut iter = InnerIndexIterator::new(Side::Ask, Some(InnerIndex::ZERO));

        // Loop through values from 1 to 31
        for i in 1..=InnerIndex::MAX.as_usize() {
            assert_eq!(iter.next(), Some(InnerIndex::new(i)));
        }

        // After reaching MAX (31), next should return None
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_inner_index_iterator_ask_start_max() {
        // Side = Ask, last_position = Some(InnerIndex::MAX)
        let mut iter = InnerIndexIterator::new(Side::Ask, Some(InnerIndex::MAX));

        // Since last_position is already MAX (31), next should return None immediately
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_inner_index_iterator_bid_start_none() {
        // Side = Bid, last_position = None
        let mut iter = InnerIndexIterator::new(Side::Bid, None);

        // Loop through values from 31 down to 0
        for i in (0..=InnerIndex::MAX.as_usize()).rev() {
            assert_eq!(iter.next(), Some(InnerIndex::new(i)));
        }

        // After reaching ZERO, next should return None
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_inner_index_iterator_bid_start_max() {
        // Side = Bid, last_position = Some(InnerIndex::MAX)
        let mut iter = InnerIndexIterator::new(Side::Bid, Some(InnerIndex::MAX));

        // Loop through values from 30 down to 0
        for i in (0..=30).rev() {
            assert_eq!(iter.next(), Some(InnerIndex::new(i)));
        }

        // After reaching ZERO, next should return None
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_inner_index_iterator_bid_start_zero() {
        // Side = Bid, last_position = Some(InnerIndex::ZERO)
        let mut iter = InnerIndexIterator::new(Side::Bid, Some(InnerIndex::ZERO));

        // Since last_position is already ZERO, next should return None immediately
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_resting_order_index_iterator_start_none() {
        // last_index = None
        let mut iter = RestingOrderIndexIterator::new(None);

        // Loop through values from 0 to 7
        for i in 0..=RestingOrderIndex::MAX.as_u8() {
            assert_eq!(iter.next(), Some(RestingOrderIndex::new(i)));
        }

        // After reaching MAX (7), next should return None
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_resting_order_index_iterator_start_zero() {
        // last_index = Some(RestingOrderIndex::ZERO)
        let mut iter = RestingOrderIndexIterator::new(Some(RestingOrderIndex::ZERO));

        // Loop through values from 1 to 7
        for i in 1..=RestingOrderIndex::MAX.as_u8() {
            assert_eq!(iter.next(), Some(RestingOrderIndex::new(i)));
        }

        // After reaching MAX (7), next should return None
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_resting_order_index_iterator_start_max() {
        // last_index = Some(RestingOrderIndex::MAX)
        let mut iter = RestingOrderIndexIterator::new(Some(RestingOrderIndex::MAX));

        // Since last_index is already MAX (7), next should return None immediately
        assert_eq!(iter.next(), None);
    }
}
