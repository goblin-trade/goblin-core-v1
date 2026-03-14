use crate::{
    axis::leg::leg_matcher::LegMatcher,
    matching::bitmap::{
        inner_pos::InnerPos, outer_bitmap_index::OuterBitmapIndex, outer_pos::OuterPos,
    },
};

#[derive(Clone, Copy)]
pub struct Range<C>
where
    C: Clone + Copy + PartialEq,
{
    pub start: C,
    pub limit: C,
}

impl<In> Range<OuterPos<In>>
where
    In: LegMatcher,
{
    pub fn adjust(
        &self,
        current: OuterBitmapIndex<In>,
        range: Range<OuterBitmapIndex<In>>,
    ) -> Self {
        Self {
            start: self.start.adjust_start(current == range.start),
            limit: self.limit.adjust_limit(current == range.limit),
        }
    }
}

impl<In> Range<InnerPos<In>>
where
    In: LegMatcher,
{
    pub fn adjust(
        &self,
        current: (OuterBitmapIndex<In>, OuterPos<In>),
        range: Range<(OuterBitmapIndex<In>, OuterPos<In>)>,
    ) -> Self {
        Self {
            start: self.start.adjust_start(current == range.start),
            limit: self.limit.adjust_limit(current == range.limit),
        }
    }
}
