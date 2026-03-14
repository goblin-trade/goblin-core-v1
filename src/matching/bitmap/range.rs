use crate::{
    axis::leg::leg_matcher::LegMatcher,
    matching::bitmap::{
        inner_pos::InnerPos, outer_bitmap_index::OuterBitmapIndex, outer_pos::OuterPos,
    },
};

#[derive(Clone, Copy)]
pub struct CustomRange<C>
where
    C: Clone + Copy + PartialEq,
{
    pub start: C,
    pub end: C,
}

impl<In> CustomRange<OuterPos<In>>
where
    In: LegMatcher,
{
    pub fn adjust(
        &self,
        current: OuterBitmapIndex<In>,
        range: CustomRange<OuterBitmapIndex<In>>,
    ) -> Self {
        Self {
            start: self.start.adjust_start(current == range.start),
            end: self.end.adjust_limit(current == range.end),
        }
    }
}

impl<In> CustomRange<InnerPos<In>>
where
    In: LegMatcher,
{
    pub fn adjust(
        &self,
        current: (OuterBitmapIndex<In>, OuterPos<In>),
        range: CustomRange<(OuterBitmapIndex<In>, OuterPos<In>)>,
    ) -> Self {
        Self {
            start: self.start.adjust_start(current == range.start),
            end: self.end.adjust_limit(current == range.end),
        }
    }
}
