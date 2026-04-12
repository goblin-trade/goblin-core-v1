use crate::{
    axis::leg::leg_matcher::LegMatcher,
    quantities::{InnerPosV2, OuterBitmapIndexV2, OuterPosV2},
};
use core::ops::RangeInclusive;

// Add inner() function
// But we need to duplicate code for each impl
pub trait OrderedIndex: Clone + Copy + PartialEq {
    type Prev: Clone + Copy + PartialEq;

    // Can add Bitmap type here or we will face recursion problem?

    fn get_iter<In>(range: RangeInclusive<Self>) -> impl Iterator<Item = Self>
    where
        In: LegMatcher;
}

impl OrderedIndex for OuterBitmapIndexV2 {
    type Prev = ();

    fn get_iter<In>(range: RangeInclusive<Self>) -> impl Iterator<Item = Self>
    where
        In: LegMatcher,
    {
        In::outer_bitmap_index_iter(range)
    }
}

impl OrderedIndex for OuterPosV2 {
    type Prev = OuterBitmapIndexV2;

    fn get_iter<In>(range: RangeInclusive<Self>) -> impl Iterator<Item = Self>
    where
        In: LegMatcher,
    {
        In::outer_pos_iter(range)
    }
}

impl OrderedIndex for InnerPosV2 {
    type Prev = OuterPosV2;

    fn get_iter<In>(range: RangeInclusive<Self>) -> impl Iterator<Item = Self>
    where
        In: LegMatcher,
    {
        In::inner_pos_iter(range)
    }
}
