use core::ops::Add;
use core::ops::AddAssign;
use core::ops::Sub;
use core::ops::SubAssign;

use crate::state::Side;
use crate::{
    quantities::{Ticks, WrapperU64},
    state::{SlotKey, BITMAP_GROUP_SEED},
};

use super::BITMAPS_PER_GROUP;

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

    /// Whether the current tick is closer to the centre than `other`
    ///
    /// # Arguments
    ///
    /// * `side`- Side of self
    /// * `other` - The other tick to compare
    pub fn is_closer_to_center(&self, side: Side, other: Ticks) -> bool {
        match side {
            // Bids are stored in descending order.
            // Current should be greater than `other`
            Side::Bid => *self > other,

            // Asks are stored in ascending order.
            // Current should be less than `other`
            Side::Ask => *self < other,
        }
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

    /// Whether the current outer index is closer to the centre than `other`
    pub fn is_closer_to_center(&self, side: Side, other: OuterIndex) -> bool {
        match side {
            // Bids are stored in descending order.
            // Current should be greater than `other`
            Side::Bid => *self > other,

            // Asks are stored in ascending order.
            // Current should be less than `other`
            Side::Ask => *self < other,
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
/// TODO replace with u8
#[derive(Clone, Copy, PartialEq, Eq, Debug, PartialOrd, Ord)]
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
        debug_assert!(inner < BITMAPS_PER_GROUP as usize);
        InnerIndex { inner }
    }

    pub fn as_usize(&self) -> usize {
        self.inner
    }

    /// Get the first inner index according to sort order for `side`.
    pub fn first(side: Side) -> InnerIndex {
        match side {
            Side::Bid => InnerIndex::MAX,  // higher to lower
            Side::Ask => InnerIndex::ZERO, // lower to higher
        }
    }

    /// Get the last inner index according to sort order for `side`.
    pub fn last(side: Side) -> InnerIndex {
        match side {
            Side::Bid => InnerIndex::ZERO, // higher to lower
            Side::Ask => InnerIndex::MAX,  // lower to higher
        }
    }

    /// Get the previous inner index according to sort order for `side`.
    ///
    /// Externally ensure that is_last() is false
    ///
    pub fn previous(&self, side: Side) -> InnerIndex {
        match side {
            Side::Bid => *self - InnerIndex::ONE, // descending order
            Side::Ask => *self + InnerIndex::ONE, // ascending order
        }
    }

    /// Get the next inner index according to sort order for `side`.
    ///
    /// Externally ensure that is_last() is false
    ///
    pub fn next(&self, side: Side) -> InnerIndex {
        match side {
            Side::Bid => *self - InnerIndex::ONE, // descending order
            Side::Ask => *self + InnerIndex::ONE, // ascending order
        }
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
