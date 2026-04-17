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
        let compliment = position.complement::<BITS>();
        let start = if compliment == range.start().complement::<BITS>() {
            Self::from(position)
        } else {
            In::start()
        };
        let end = if compliment == range.end().complement::<BITS>() {
            Self::from(position)
        } else {
            In::end()
        };

        start..=end
    }
}
