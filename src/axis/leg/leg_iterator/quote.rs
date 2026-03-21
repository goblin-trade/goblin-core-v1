use core::iter::Map;
use core::ops::RangeInclusive;

use crate::{
    axis::leg::{leg_iterator::LegIterator, Quote},
    matching::bitmap::{
        inner_pos::InnerPos, outer_bitmap_index::OuterBitmapIndex, outer_pos::OuterPos, row::Row,
    },
};

impl LegIterator for Quote {
    type OuterBitmapIndexIter = Map<RangeInclusive<u64>, fn(u64) -> OuterBitmapIndex>;
    type OuterPosIter = Map<RangeInclusive<u8>, fn(u8) -> OuterPos>;
    type InnerPosIter = Map<RangeInclusive<u8>, fn(u8) -> InnerPos>;

    type RowIter = Map<RangeInclusive<u8>, fn(u8) -> Row>;

    fn outer_bitmap_index_iter(
        range: RangeInclusive<OuterBitmapIndex>,
    ) -> Self::OuterBitmapIndexIter {
        (range.start().inner..=range.end().inner).map(OuterBitmapIndex::new)
    }

    fn outer_pos_iter(range: RangeInclusive<OuterPos>) -> Self::OuterPosIter {
        (range.start().inner..=range.end().inner).map(OuterPos::new)
    }

    fn inner_pos_iter(range: RangeInclusive<InnerPos>) -> Self::InnerPosIter {
        (range.start().inner..=range.end().inner).map(InnerPos::new)
    }

    fn row_iter(range: RangeInclusive<Row>) -> Self::RowIter {
        (range.start().inner..=range.end().inner).map(Row::new)
    }
}
