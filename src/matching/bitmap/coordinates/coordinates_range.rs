use crate::{
    axis::leg::leg_matcher::LegMatcher,
    matching::bitmap::{
        inner_pos::InnerPos, outer_bitmap_index::OuterBitmapIndex, outer_pos::OuterPos,
        FullCoordinates,
    },
};
use core::ops::RangeInclusive;

pub struct CoordinatesRange<In>
where
    In: LegMatcher,
{
    pub outer_bitmap_index: RangeInclusive<OuterBitmapIndex<In>>,
    pub outer_pos: RangeInclusive<OuterPos<In>>,
    pub inner_pos: RangeInclusive<InnerPos<In>>,
}

impl<In> From<RangeInclusive<FullCoordinates<In>>> for CoordinatesRange<In>
where
    In: LegMatcher,
{
    fn from(value: RangeInclusive<FullCoordinates<In>>) -> Self {
        Self {
            outer_bitmap_index: value.start().outer_bitmap_index..=value.end().outer_bitmap_index,
            outer_pos: value.start().outer_pos..=value.end().outer_pos,
            inner_pos: value.start().inner_pos..=value.end().inner_pos,
        }
    }
}
