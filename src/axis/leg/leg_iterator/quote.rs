use core::iter::Map;
use core::ops::RangeInclusive;

use crate::{
    axis::leg::{leg_iterator::LegIterator, Quote},
    matching::bitmap::{
        inner_pos::InnerPos, outer_bitmap_index::OuterBitmapIndex, outer_pos::OuterPos, row::Row,
        Coordinate,
    },
};

impl LegIterator for Quote {
    type OuterBitmapIndexIter = Map<RangeInclusive<u64>, fn(u64) -> OuterBitmapIndex<Self>>;
    type OuterPosIter = Map<RangeInclusive<u8>, fn(u8) -> OuterPos<Self>>;
    type InnerPosIter = Map<RangeInclusive<u8>, fn(u8) -> InnerPos<Self>>;

    type RowIter = Map<RangeInclusive<u8>, fn(u8) -> Row<Self>>;

    fn outer_bitmap_index_iter(item: OuterBitmapIndex<Self>) -> Self::OuterBitmapIndexIter {
        (item.inner..=OuterBitmapIndex::<Self>::MAX.inner).map(OuterBitmapIndex::new)
    }

    fn outer_pos_iter(item: OuterPos<Self>) -> Self::OuterPosIter {
        (item.inner..=OuterPos::<Self>::MAX.inner).map(OuterPos::new)
    }

    fn inner_pos_iter(item: InnerPos<Self>) -> Self::InnerPosIter {
        (item.inner..=255).map(InnerPos::new)
    }

    fn row_iter(item: Row<Self>) -> Self::RowIter {
        (item.inner..=Row::<Self>::MAX.inner).map(Row::new)
    }
}
