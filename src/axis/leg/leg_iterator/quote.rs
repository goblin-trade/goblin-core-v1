use core::iter::Map;
use core::ops::RangeInclusive;

use crate::{
    axis::leg::{leg_iterator::LegIterator, Quote},
    matching::bitmap::{
        compact_coordinates::CompactCoordinates, outer_bitmap_index::OuterBitmapIndex,
        outer_pos::OuterPos, row::Row, Coordinate,
    },
};

impl LegIterator for Quote {
    type OuterBitmapIndexIter = Map<RangeInclusive<u64>, fn(u64) -> OuterBitmapIndex<Self>>;
    // type Iterable<C: Coordinate> = Map<RangeInclusive<C::Inner>, fn(C::Inner) -> C>;

    fn outer_bitmap_index_iter(item: OuterBitmapIndex<Self>) -> Self::OuterBitmapIndexIter {
        (item.inner..=OuterBitmapIndex::<Self>::MAX.inner).map(OuterBitmapIndex::new)
    }

    fn outer_pos_iter(item: OuterPos<Self>) -> impl Iterator<Item = OuterPos<Self>> {
        (item.inner..=OuterPos::<Self>::MAX.inner).map(OuterPos::new)
    }

    fn row_iter(item: Row<Self>) -> impl Iterator<Item = Row<Self>> {
        (item.inner..=Row::<Self>::MAX.inner).map(Row::new)
    }

    fn coordinates_iter(
        item: CompactCoordinates<Self>,
    ) -> impl Iterator<Item = CompactCoordinates<Self>> {
        (item.inner..=255).map(CompactCoordinates::new)
    }
}
