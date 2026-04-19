use crate::axis::leg::{leg_iterator::LegIterator, Base};
use crate::quantities::{
    Position, PositionRange, INNER_POS_V2, OUTER_BITMAP_INDEX_V2, OUTER_POS_V2,
};
use core::iter::{Map, Rev};
use core::ops::RangeInclusive;

impl LegIterator for Base {
    type PositionIter = Map<Rev<RangeInclusive<u64>>, fn(u64) -> Position>;

    // type RowIter = Map<Rev<RangeInclusive<u8>>, fn(u8) -> Row>;

    // fn row_iter(range: RangeInclusive<Row>) -> Self::RowIter {
    //     (range.end().inner..=range.start().inner)
    //         .rev()
    //         .map(Row::new)
    // }

    fn outer_bitmap_index_iter(range: RangeInclusive<Position>) -> Self::PositionIter {
        let extracted_range = range.extract_range::<OUTER_BITMAP_INDEX_V2>();
        (extracted_range.end().inner..=extracted_range.start().inner)
            .rev()
            .map(Position::new)
    }

    fn outer_pos_iter(range: RangeInclusive<Position>, current: Position) -> Self::PositionIter {
        let extracted_range = range.effective_range::<Self, OUTER_POS_V2>(current);
        (extracted_range.end().inner..=extracted_range.start().inner)
            .rev()
            .map(Position::new)
    }

    fn inner_pos_iter(range: RangeInclusive<Position>, current: Position) -> Self::PositionIter {
        /// Mask inverts the LSB 3 bits belonging to column
        /// Eg the starting value 255 will map to 248 (row 31, column 0).
        /// This way rows are traversed top to bottom as normal but
        /// the direction of column traversal becomes left to right.
        const LSB3_MASK: u64 = 0b111;

        let extracted_range = range.effective_range::<Self, INNER_POS_V2>(current);

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
