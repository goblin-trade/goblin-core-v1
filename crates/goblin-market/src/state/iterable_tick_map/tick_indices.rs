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
}

/// Key to fetch a Bitmap group. A Bitmap consists of multiple Bitmaps
#[derive(Clone, Copy, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct OuterIndex {
    /// Index of bitmap group
    inner: u16,
}

impl OuterIndex {
    pub fn new(inner: u16) -> Self {
        OuterIndex { inner }
    }

    pub fn as_u16(&self) -> u16 {
        self.inner
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
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct InnerIndex {
    /// Relative position of the bitmap within the bitmap group
    inner: usize,
}

impl InnerIndex {
    // TODO replace assert
    pub fn new(inner: usize) -> Self {
        assert!(inner < BITMAPS_PER_GROUP as usize);
        InnerIndex { inner }
    }

    pub fn as_usize(&self) -> usize {
        self.inner
    }
}
