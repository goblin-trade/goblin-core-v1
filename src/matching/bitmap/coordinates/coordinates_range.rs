use crate::{
    axis::leg::leg_matcher::LegMatcher,
    matching::bitmap::{
        inner_pos::InnerPos, outer_bitmap_index::OuterBitmapIndex, outer_pos::OuterPos,
        range::Range, FullCoordinates,
    },
};

pub struct CoordinatesRange<In>
where
    In: LegMatcher,
{
    pub outer_bitmap_index: Range<OuterBitmapIndex<In>>,
    pub outer_pos: Range<OuterPos<In>>,
    pub inner_pos: Range<InnerPos<In>>,
}

impl<In> From<Range<FullCoordinates<In>>> for CoordinatesRange<In>
where
    In: LegMatcher,
{
    fn from(value: Range<FullCoordinates<In>>) -> Self {
        Self {
            outer_bitmap_index: Range {
                start: value.start.outer_bitmap_index,
                limit: value.limit.outer_bitmap_index,
            },
            outer_pos: Range {
                start: value.start.outer_pos,
                limit: value.limit.outer_pos,
            },
            inner_pos: Range {
                start: value.start.inner_pos,
                limit: value.limit.inner_pos,
            },
        }
    }
}
