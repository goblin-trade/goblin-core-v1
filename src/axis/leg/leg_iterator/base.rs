use core::iter::{Map, Rev};
use core::ops::RangeInclusive;

use crate::{
    axis::leg::{leg_iterator::LegIterator, Base},
    matching::bitmap::{
        compact_coordinates::CompactCoordinates, outer_bitmap_index::OuterBitmapIndex,
        outer_pos::OuterPos, row::Row, Coordinate,
    },
};

impl LegIterator for Base {
    type Iterable<C: Coordinate> = Map<Rev<RangeInclusive<C::Inner>>, fn(C::Inner) -> C>;

    fn outer_bitmap_index_iter(
        item: OuterBitmapIndex<Self>,
    ) -> impl Iterator<Item = OuterBitmapIndex<Self>> {
        (OuterBitmapIndex::<Self>::MIN.inner..=item.inner)
            .rev()
            .map(OuterBitmapIndex::<Self>::new)
    }

    fn outer_pos_iter(item: OuterPos<Self>) -> impl Iterator<Item = OuterPos<Self>> {
        (OuterPos::<Self>::MIN.inner..=item.inner)
            .rev()
            .map(OuterPos::<Self>::new)
    }

    fn row_iter(item: Row<Self>) -> impl Iterator<Item = Row<Self>> {
        (Row::<Self>::MIN.inner..=item.inner)
            .rev()
            .map(Row::<Self>::new)
    }

    fn coordinates_iter(
        item: CompactCoordinates<Self>,
    ) -> impl Iterator<Item = CompactCoordinates<Self>> {
        /// Mask inverts the LSB 3 bits belonging to column
        /// Eg the starting value 255 will map to 248 (row 31, column 0).
        /// This way rows are traversed top to bottom as normal but
        /// the direction of column traversal becomes left to right.
        const LSB3_MASK: u8 = 0b0000_0111;

        // Invert the starting bits. This way they get inverted again
        // to the original value inside map()
        let item_inverted = item.inner ^ LSB3_MASK;

        (0..=item_inverted).rev().map(|inner| {
            let inner_inverted = inner ^ LSB3_MASK;
            CompactCoordinates::new(inner_inverted)
        })
    }
}
