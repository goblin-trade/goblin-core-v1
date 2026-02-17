use crate::{
    axis::leg::leg_matcher::LegMatcher,
    matching::bitmap::{outer_bitmap_index::OuterBitmapIndex, outer_pos::OuterPos, row::Row},
    quantities::Ticks,
};

/// Coordinate representation of a price tick
pub struct PriceCoordinates<In>
where
    In: LegMatcher,
{
    pub outer_bitmap_index: OuterBitmapIndex<In>,
    pub outer_pos: OuterPos<In>,
    pub row: Row<In>,
}

impl<In> From<Ticks> for PriceCoordinates<In>
where
    In: LegMatcher,
{
    fn from(value: Ticks) -> Self {
        Self {
            outer_bitmap_index: value.into(),
            outer_pos: value.into(),
            row: value.into(),
        }
    }
}

impl<In> From<PriceCoordinates<In>> for Ticks
where
    In: LegMatcher,
{
    fn from(value: PriceCoordinates<In>) -> Self {
        let outer_bitmap_index = value.outer_bitmap_index.inner;
        let outer_pos = value.outer_pos.inner as u64;
        let row = value.row.inner as u64;

        let inner = outer_bitmap_index * (256 * 32) + outer_pos * 32 + row;

        Self::new(inner)
    }
}
