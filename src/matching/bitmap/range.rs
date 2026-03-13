use crate::{
    axis::leg::leg_matcher::LegMatcher,
    matching::bitmap::{outer_bitmap_index::OuterBitmapIndex, outer_pos::OuterPos},
};

#[derive(Clone, Copy)]
pub struct Range<C>
where
    C: Clone + Copy + PartialEq,
{
    pub start: C,
    pub limit: C,
}

impl<C> Range<C>
where
    C: Clone + Copy + PartialEq,
{
    /// Whether start equals limit
    ///
    /// For OuterPos and Row, we additionally need to check the top level dimensions.
    ///
    /// We compare OuterPos only if we are on the same OuterBitmapIndex
    pub fn on_limit(&self) -> bool {
        self.start == self.limit
    }
}

impl<In> Range<OuterPos<In>>
where
    In: LegMatcher,
{
    pub fn adjust(&self, outer_bitmap_index_range: Range<OuterBitmapIndex<In>>) -> Self {
        Self {
            start: self.start.get_start(outer_bitmap_index_range),
            limit: self.start.get_limit(outer_bitmap_index_range),
        }
    }
}
