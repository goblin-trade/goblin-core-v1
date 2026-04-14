use core::ops::RangeInclusive;

use crate::{
    axis::leg::leg_matcher::LegMatcher,
    quantities::DerivedPosition,
    state::bitmap_v2::{ordered_index::OrderedIndex, outer_index::OuterIndex},
};

pub type BitmapIndexV2<const BITS: usize> = DerivedPosition<u8, BITS>;

impl<const BITS: usize> BitmapIndexV2<BITS>
where
    Self: OrderedIndex,
    // RangeInclusive<(OuterIndex<Self>, Self)>: Clone + Copy,
{
    pub fn byte_index(&self) -> usize {
        self.inner as usize / 8
    }

    pub fn bit_index(&self) -> usize {
        self.inner as usize % 8
    }

    pub fn clamped_range<In>(
        range: RangeInclusive<(OuterIndex<Self>, Self)>,
        current_outer: OuterIndex<Self>,
    ) -> RangeInclusive<Self>
    where
        In: LegMatcher,
        // RangeInclusive<(OuterIndex<Self>, Self)>: Clone + Copy,
    {
        let start = if current_outer == range.start().0 {
            range.start().1
        } else {
            In::start()
        };
        let end = if current_outer == range.end().0 {
            range.end().1
        } else {
            In::end()
        };

        start..=end
    }
}
