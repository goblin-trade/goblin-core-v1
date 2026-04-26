use crate::axis::leg::{leg_iterator::LegIterator, Base};
use crate::quantities::bits_layout::BitsLayout;
use crate::quantities::{Position, PositionRange, INNER_POS_V2};
use core::iter::{Map, Rev, StepBy};
use core::ops::RangeInclusive;

impl LegIterator for Base {
    type PositionIter = Map<StepBy<Rev<RangeInclusive<u64>>>, fn(u64) -> Position>;

    fn step_iter<const BITS: u16>(range: RangeInclusive<Position>) -> Self::PositionIter {
        let extracted_range = range.extract_range::<BITS>();
        (extracted_range.end().inner..=extracted_range.start().inner)
            .rev()
            .step_by(BitsLayout::<BITS>::step_interval())
            .map(Position::new)
    }

    fn inner_pos_iter(range: RangeInclusive<Position>, current: Position) -> Self::PositionIter {
        /// Mask inverts the LSB 3 bits belonging to column
        /// Eg the starting value 255 will map to 248 (row 31, column 0).
        /// This way rows are traversed top to bottom as normal but
        /// the direction of column traversal becomes left to right.
        const INVERT_COLUMN_MASK: u64 = 0b111;

        let extracted_range = range.clamp_range::<Self, INNER_POS_V2>(current);

        // Invert the starting bits. This way they get inverted again
        // to the original value inside map()
        let start_inverted = extracted_range.start().inner ^ INVERT_COLUMN_MASK;

        (extracted_range.end().inner..=start_inverted)
            .rev()
            .step_by(BitsLayout::<INNER_POS_V2>::step_interval())
            .map(|inner| {
                let inner_inverted = inner ^ INVERT_COLUMN_MASK;
                Position::new(inner_inverted)
            })
    }
}
