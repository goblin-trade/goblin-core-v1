use core::ops::RangeInclusive;

use crate::{
    axis::leg::leg_matcher::LegMatcher,
    quantities::{DerivedPosition, Position},
    state::bitmap_v2::ordered_index::OrderedIndex,
};

pub type BitmapIndexV2<const BITS: u16> = DerivedPosition<u8, BITS>;

impl<const BITS: u16> BitmapIndexV2<BITS>
where
    Self: OrderedIndex,
{
    pub fn byte_index(&self) -> usize {
        self.inner as usize / 8
    }

    pub fn bit_index(&self) -> usize {
        self.inner as usize % 8
    }

    pub fn effective_range<In>(
        range: &RangeInclusive<Position>,
        position: Position,
    ) -> RangeInclusive<Self>
    where
        In: LegMatcher,
    {
        let offset = Self::BIT_OFFSET as usize;
        let compliment = position.complement(offset);
        let start = if compliment == range.start().complement(offset) {
            Self::from(position)
        } else {
            In::start()
        };
        let end = if compliment == range.end().complement(offset) {
            Self::from(position)
        } else {
            In::end()
        };

        start..=end
    }

    pub fn clamped_range<In>(
        range: &RangeInclusive<(OuterIndex<Self>, Self)>,
        current_outer: OuterIndex<Self>,
    ) -> RangeInclusive<Self>
    where
        In: LegMatcher,
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
