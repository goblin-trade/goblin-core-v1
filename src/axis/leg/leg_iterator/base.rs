use crate::axis::leg::{leg_iterator::LegIterator, Base};
use crate::quantities::{
    InnerPosV2, OuterBitmapIndexV2, OuterPosV2, Position, PositionRange, INNER_POS_V2,
    OUTER_BITMAP_INDEX_V2, OUTER_POS_V2,
};
use core::iter::{Map, Rev};
use core::ops::RangeInclusive;

impl LegIterator for Base {
    type OuterBitmapIndexIter = Map<Rev<RangeInclusive<u64>>, fn(u64) -> OuterBitmapIndexV2>;
    type OuterPosIter = Map<Rev<RangeInclusive<u8>>, fn(u8) -> OuterPosV2>;
    type InnerPosIter = Map<Rev<RangeInclusive<u8>>, fn(u8) -> InnerPosV2>;
    type PositionIter = Map<Rev<RangeInclusive<u64>>, fn(u64) -> Position>;

    // type RowIter = Map<Rev<RangeInclusive<u8>>, fn(u8) -> Row>;

    fn outer_bitmap_index_iter(
        range: RangeInclusive<OuterBitmapIndexV2>,
    ) -> Self::OuterBitmapIndexIter {
        (range.end().inner..=range.start().inner)
            .rev()
            .map(OuterBitmapIndexV2::new)
    }

    fn outer_pos_iter(range: RangeInclusive<OuterPosV2>) -> Self::OuterPosIter {
        (range.end().inner..=range.start().inner)
            .rev()
            .map(OuterPosV2::new)
    }

    fn inner_pos_iter(range: RangeInclusive<InnerPosV2>) -> Self::InnerPosIter {
        /// Mask inverts the LSB 3 bits belonging to column
        /// Eg the starting value 255 will map to 248 (row 31, column 0).
        /// This way rows are traversed top to bottom as normal but
        /// the direction of column traversal becomes left to right.
        const LSB3_MASK: u8 = 0b0000_0111;

        // Invert the starting bits. This way they get inverted again
        // to the original value inside map()
        let item_inverted = range.start().inner ^ LSB3_MASK;

        (range.end().inner..=item_inverted).rev().map(|inner| {
            let inner_inverted = inner ^ LSB3_MASK;
            InnerPosV2::new(inner_inverted)
        })
    }

    // fn row_iter(range: RangeInclusive<Row>) -> Self::RowIter {
    //     (range.end().inner..=range.start().inner)
    //         .rev()
    //         .map(Row::new)
    // }

    fn outer_bitmap_index_iter_v2(range: RangeInclusive<Position>) -> Self::PositionIter {
        let extracted_range = range.extract_range::<OUTER_BITMAP_INDEX_V2>();
        (extracted_range.end().inner..=extracted_range.start().inner)
            .rev()
            .map(Position::new)
    }

    fn outer_pos_iter_v2(range: RangeInclusive<Position>) -> Self::PositionIter {
        let extracted_range = range.extract_range::<OUTER_POS_V2>();
        (extracted_range.end().inner..=extracted_range.start().inner)
            .rev()
            .map(Position::new)
    }

    fn inner_pos_iter_v2(range: RangeInclusive<Position>) -> Self::PositionIter {
        /// Mask inverts the LSB 3 bits belonging to column
        /// Eg the starting value 255 will map to 248 (row 31, column 0).
        /// This way rows are traversed top to bottom as normal but
        /// the direction of column traversal becomes left to right.
        const LSB3_MASK: u64 = 0b111;

        let extracted_range = range.extract_range::<INNER_POS_V2>();

        // Invert the starting bits. This way they get inverted again
        // to the original value inside map()
        let item_inverted = extracted_range.start().inner ^ LSB3_MASK;

        (extracted_range.end().inner..=item_inverted)
            .rev()
            .map(|inner| {
                let inner_inverted = inner ^ LSB3_MASK;
                Position::new(inner_inverted)
            })
    }
}
