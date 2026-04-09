use core::ops::RangeInclusive;

use crate::{
    axis::leg::leg_matcher::LegMatcher,
    quantities::{InnerPosV2, OuterBitmapIndexV2, OuterPosV2},
};

// Add inner() function
// But we need to duplicate code for each impl
pub trait OrderedIndex: Clone + Copy + PartialEq {
    type Outer: Clone + Copy + PartialEq;

    fn get_iter<In>(range: RangeInclusive<Self>) -> impl Iterator<Item = Self>
    where
        In: LegMatcher;
}

pub type OuterIndex<I: OrderedIndex> = (<I::Outer as OrderedIndex>::Outer, I::Outer);

impl OrderedIndex for OuterBitmapIndexV2 {
    type Outer = ();

    fn get_iter<In>(range: RangeInclusive<Self>) -> impl Iterator<Item = Self>
    where
        In: LegMatcher,
    {
        In::outer_bitmap_index_iter(range)
    }
}

impl OrderedIndex for OuterPosV2 {
    type Outer = OuterBitmapIndexV2;

    fn get_iter<In>(range: RangeInclusive<Self>) -> impl Iterator<Item = Self>
    where
        In: LegMatcher,
    {
        In::outer_pos_iter(range)
    }
}

impl OrderedIndex for InnerPosV2 {
    type Outer = OuterPosV2;

    fn get_iter<In>(range: RangeInclusive<Self>) -> impl Iterator<Item = Self>
    where
        In: LegMatcher,
    {
        In::inner_pos_iter(range)
    }
}
