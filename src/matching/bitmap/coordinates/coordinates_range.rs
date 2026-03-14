use crate::{
    axis::leg::leg_matcher::LegMatcher,
    matching::bitmap::{
        inner_pos::InnerPos, outer_bitmap_index::OuterBitmapIndex, outer_pos::OuterPos,
        range::CustomRange, FullCoordinates,
    },
};

pub struct CoordinatesRange<In>
where
    In: LegMatcher,
{
    pub outer_bitmap_index: CustomRange<OuterBitmapIndex<In>>,
    pub outer_pos: CustomRange<OuterPos<In>>,
    pub inner_pos: CustomRange<InnerPos<In>>,
}

impl<In> From<CustomRange<FullCoordinates<In>>> for CoordinatesRange<In>
where
    In: LegMatcher,
{
    fn from(value: CustomRange<FullCoordinates<In>>) -> Self {
        Self {
            outer_bitmap_index: CustomRange {
                start: value.start.outer_bitmap_index,
                end: value.end.outer_bitmap_index,
            },
            outer_pos: CustomRange {
                start: value.start.outer_pos,
                end: value.end.outer_pos,
            },
            inner_pos: CustomRange {
                start: value.start.inner_pos,
                end: value.end.inner_pos,
            },
        }
    }
}
