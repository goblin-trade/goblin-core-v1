use core::iter::{Map, Rev};
use core::ops::RangeInclusive;

use crate::matching::bitmap::range::CustomRange;
use crate::{
    axis::leg::{leg_iterator::LegIterator, Base},
    matching::bitmap::{
        inner_pos::InnerPos, outer_bitmap_index::OuterBitmapIndex, outer_pos::OuterPos, row::Row,
    },
};

impl LegIterator for Base {
    type OuterBitmapIndexIter = Map<Rev<RangeInclusive<u64>>, fn(u64) -> OuterBitmapIndex<Self>>;
    type OuterPosIter = Map<Rev<RangeInclusive<u8>>, fn(u8) -> OuterPos<Self>>;
    type InnerPosIter = Map<Rev<RangeInclusive<u8>>, fn(u8) -> InnerPos<Self>>;

    type RowIter = Map<Rev<RangeInclusive<u8>>, fn(u8) -> Row<Self>>;

    fn outer_bitmap_index_iter(
        range: CustomRange<OuterBitmapIndex<Self>>,
    ) -> Self::OuterBitmapIndexIter {
        (range.end.inner..=range.start.inner)
            .rev()
            .map(OuterBitmapIndex::<Self>::new)
    }

    fn outer_pos_iter(range: CustomRange<OuterPos<Self>>) -> Self::OuterPosIter {
        (range.end.inner..=range.start.inner)
            .rev()
            .map(OuterPos::<Self>::new)
    }

    fn inner_pos_iter(range: CustomRange<InnerPos<Self>>) -> Self::InnerPosIter {
        /// Mask inverts the LSB 3 bits belonging to column
        /// Eg the starting value 255 will map to 248 (row 31, column 0).
        /// This way rows are traversed top to bottom as normal but
        /// the direction of column traversal becomes left to right.
        const LSB3_MASK: u8 = 0b0000_0111;

        // Invert the starting bits. This way they get inverted again
        // to the original value inside map()
        let item_inverted = range.start.inner ^ LSB3_MASK;

        (range.end.inner..=item_inverted).rev().map(|inner| {
            let inner_inverted = inner ^ LSB3_MASK;
            InnerPos::new(inner_inverted)
        })
    }

    fn row_iter(range: CustomRange<Row<Self>>) -> Self::RowIter {
        (range.end.inner..=range.start.inner)
            .rev()
            .map(Row::<Self>::new)
    }
}
