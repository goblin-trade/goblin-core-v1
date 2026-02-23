use crate::{
    axis::leg::leg_matcher::LegMatcher,
    matching::bitmap::{
        outer_bitmap_index::OuterBitmapIndex, outer_pos::OuterPos, range::Range, row::Row,
        PriceCoordinates,
    },
    quantities::Ticks,
};

pub struct CoordinatesRange<In>
where
    In: LegMatcher,
{
    pub outer_bitmap_index: Range<OuterBitmapIndex<In>>,
    pub outer_pos: Range<OuterPos<In>>,
    pub row: Range<Row<In>>,
}

impl<In> From<Range<Ticks>> for CoordinatesRange<In>
where
    In: LegMatcher,
{
    fn from(value: Range<Ticks>) -> Self {
        let start_coordinates = PriceCoordinates::from(value.start);
        let limit_coordinates = PriceCoordinates::from(value.limit);

        Self {
            outer_bitmap_index: Range {
                start: start_coordinates.outer_bitmap_index,
                limit: limit_coordinates.outer_bitmap_index,
            },
            outer_pos: Range {
                start: start_coordinates.outer_pos,
                limit: limit_coordinates.outer_pos,
            },
            row: Range {
                start: start_coordinates.row,
                limit: limit_coordinates.row,
            },
        }
    }
}
