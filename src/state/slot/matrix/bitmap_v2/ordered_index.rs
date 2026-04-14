use crate::{
    axis::leg::leg_matcher::LegMatcher,
    quantities::{InnerPosV2, OuterBitmapIndexV2, OuterPosV2},
};
use core::ops::RangeInclusive;

// Add inner() function
// But we need to duplicate code for each impl
pub trait OrderedIndex: Clone + Copy + PartialEq {
    type Prev: OrderedIndex<Prev: Clone + Copy + PartialEq>;

    fn linear_iterator<In>(range: RangeInclusive<Self>) -> impl Iterator<Item = Self>
    where
        In: LegMatcher;
}

impl OrderedIndex for () {
    type Prev = (); // bottoms out, self-referential terminator

    fn linear_iterator<In>(_range: RangeInclusive<Self>) -> impl Iterator<Item = Self>
    where
        In: LegMatcher,
    {
        core::iter::once(()) // or empty(), depending on your semantics
    }
}

impl OrderedIndex for OuterBitmapIndexV2 {
    type Prev = ();

    fn linear_iterator<In>(range: RangeInclusive<Self>) -> impl Iterator<Item = Self>
    where
        In: LegMatcher,
    {
        In::outer_bitmap_index_iter(range)
    }
}

impl OrderedIndex for OuterPosV2 {
    type Prev = OuterBitmapIndexV2;

    fn linear_iterator<In>(range: RangeInclusive<Self>) -> impl Iterator<Item = Self>
    where
        In: LegMatcher,
    {
        In::outer_pos_iter(range)
    }
}

impl OrderedIndex for InnerPosV2 {
    type Prev = OuterPosV2;

    fn linear_iterator<In>(range: RangeInclusive<Self>) -> impl Iterator<Item = Self>
    where
        In: LegMatcher,
    {
        In::inner_pos_iter(range)
    }
}
