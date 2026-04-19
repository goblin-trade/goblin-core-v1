use crate::{
    axis::leg::{leg_iterator::LegIterator, Quote},
    quantities::{Position, PositionRange, INNER_POS_V2, OUTER_BITMAP_INDEX_V2, OUTER_POS_V2},
};
use core::iter::Map;
use core::ops::RangeInclusive;

impl LegIterator for Quote {
    type PositionIter = Map<RangeInclusive<u64>, fn(u64) -> Position>;

    // type RowIter = Map<RangeInclusive<u8>, fn(u8) -> Row>;
    // fn row_iter(range: RangeInclusive<Row>) -> Self::RowIter {
    //     (range.start().inner..=range.end().inner).map(Row::new)
    // }

    fn outer_bitmap_index_iter(range: RangeInclusive<Position>) -> Self::PositionIter {
        let extracted_range = range.extract_range::<OUTER_BITMAP_INDEX_V2>();
        (extracted_range.start().inner..=extracted_range.end().inner).map(Position::new)
    }

    fn outer_pos_iter(range: RangeInclusive<Position>) -> Self::PositionIter {
        let extracted_range = range.extract_range::<OUTER_POS_V2>();
        (extracted_range.start().inner..=extracted_range.end().inner).map(Position::new)
    }

    fn inner_pos_iter(range: RangeInclusive<Position>) -> Self::PositionIter {
        let extracted_range = range.extract_range::<INNER_POS_V2>();
        (extracted_range.start().inner..=extracted_range.end().inner).map(Position::new)
    }
}
