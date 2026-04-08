use core::ops::RangeInclusive;

use crate::{
    axis::leg::leg_matcher::LegMatcher,
    quantities::{InnerPosV2, OuterBitmapIndexV2, OuterPosV2},
};

// Add inner() function
// But we need to duplicate code for each impl
pub trait Index: Clone + Copy + PartialEq {
    type Inner;
    type Outer: Clone + Copy + PartialEq;

    fn inner(&self) -> Self::Inner;

    fn get_iter<In>(range: RangeInclusive<Self>) -> impl Iterator<Item = Self>
    where
        In: LegMatcher;
}

pub type OuterIndex<I: Index> = (<I::Outer as Index>::Outer, I::Outer);

impl Index for OuterBitmapIndexV2 {
    type Inner = u64;
    type Outer = ();

    fn inner(&self) -> Self::Inner {
        self.inner
    }

    fn get_iter<In>(range: RangeInclusive<Self>) -> impl Iterator<Item = Self>
    where
        In: LegMatcher,
    {
        In::outer_bitmap_index_iter(range)
    }
}

impl Index for OuterPosV2 {
    type Inner = u8;
    type Outer = OuterBitmapIndexV2;

    fn inner(&self) -> Self::Inner {
        self.inner
    }

    fn get_iter<In>(range: RangeInclusive<Self>) -> impl Iterator<Item = Self>
    where
        In: LegMatcher,
    {
        In::outer_pos_iter(range)
    }
}

impl Index for InnerPosV2 {
    type Inner = u8;
    type Outer = OuterPosV2;

    fn inner(&self) -> Self::Inner {
        self.inner
    }

    fn get_iter<In>(range: RangeInclusive<Self>) -> impl Iterator<Item = Self>
    where
        In: LegMatcher,
    {
        In::inner_pos_iter(range)
    }
}
