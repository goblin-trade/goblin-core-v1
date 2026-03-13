use core::iter::Map;
use core::ops::RangeInclusive;

use crate::{
    axis::leg::{leg_iterator::LegIterator, Quote},
    matching::bitmap::{
        inner_pos::InnerPos, outer_bitmap_index::OuterBitmapIndex, outer_pos::OuterPos,
        range::Range, row::Row,
    },
};

impl LegIterator for Quote {
    type OuterBitmapIndexIter = Map<RangeInclusive<u64>, fn(u64) -> OuterBitmapIndex<Self>>;
    type OuterPosIter = Map<RangeInclusive<u8>, fn(u8) -> OuterPos<Self>>;
    type InnerPosIter = Map<RangeInclusive<u8>, fn(u8) -> InnerPos<Self>>;

    type RowIter = Map<RangeInclusive<u8>, fn(u8) -> Row<Self>>;

    fn outer_bitmap_index_iter(range: Range<OuterBitmapIndex<Self>>) -> Self::OuterBitmapIndexIter {
        (range.start.inner..=range.limit.inner).map(OuterBitmapIndex::new)
    }

    fn outer_pos_iter(range: Range<OuterPos<Self>>) -> Self::OuterPosIter {
        (range.start.inner..=range.limit.inner).map(OuterPos::new)
    }

    fn inner_pos_iter(range: Range<InnerPos<Self>>) -> Self::InnerPosIter {
        (range.start.inner..=range.limit.inner).map(InnerPos::new)
    }

    fn row_iter(range: Range<Row<Self>>) -> Self::RowIter {
        (range.start.inner..=range.limit.inner).map(Row::new)
    }
}
