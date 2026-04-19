use core::iter::Map;
use core::ops::RangeInclusive;

use crate::{
    axis::leg::{leg_iterator::LegIterator, Quote},
    quantities::{
        InnerPosV2, OuterBitmapIndexV2, OuterPosV2, Position, PositionRange, INNER_POS_V2,
        OUTER_BITMAP_INDEX_V2, OUTER_POS_V2,
    },
};

impl LegIterator for Quote {
    type OuterBitmapIndexIter = Map<RangeInclusive<u64>, fn(u64) -> OuterBitmapIndexV2>;
    type OuterPosIter = Map<RangeInclusive<u8>, fn(u8) -> OuterPosV2>;
    type InnerPosIter = Map<RangeInclusive<u8>, fn(u8) -> InnerPosV2>;
    type PositionIter = Map<RangeInclusive<u64>, fn(u64) -> Position>;

    // type RowIter = Map<RangeInclusive<u8>, fn(u8) -> Row>;

    fn outer_bitmap_index_iter(
        range: RangeInclusive<OuterBitmapIndexV2>,
    ) -> Self::OuterBitmapIndexIter {
        (range.start().inner..=range.end().inner).map(OuterBitmapIndexV2::new)
    }

    fn outer_pos_iter(range: RangeInclusive<OuterPosV2>) -> Self::OuterPosIter {
        (range.start().inner..=range.end().inner).map(OuterPosV2::new)
    }

    fn inner_pos_iter(range: RangeInclusive<InnerPosV2>) -> Self::InnerPosIter {
        (range.start().inner..=range.end().inner).map(InnerPosV2::new)
    }

    // fn row_iter(range: RangeInclusive<Row>) -> Self::RowIter {
    //     (range.start().inner..=range.end().inner).map(Row::new)
    // }

    fn outer_bitmap_index_iter_v2(range: RangeInclusive<Position>) -> Self::PositionIter {
        let extracted_range = range.extract_range::<OUTER_BITMAP_INDEX_V2>();
        (extracted_range.start().inner..=extracted_range.end().inner).map(Position::new)
    }

    fn outer_pos_iter_v2(range: RangeInclusive<Position>) -> Self::PositionIter {
        let extracted_range = range.extract_range::<OUTER_POS_V2>();
        (extracted_range.start().inner..=extracted_range.end().inner).map(Position::new)
    }

    fn inner_pos_iter_v2(range: RangeInclusive<Position>) -> Self::PositionIter {
        let extracted_range = range.extract_range::<INNER_POS_V2>();
        (extracted_range.start().inner..=extracted_range.end().inner).map(Position::new)
    }
}
