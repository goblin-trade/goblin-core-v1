use crate::axis::leg::{leg_iterator::LegIterator, Base};
use crate::quantities::bits_layout::BitsLayout;
use crate::quantities::{Position, PositionRange};
use core::iter::{Map, Rev, StepBy};
use core::ops::RangeInclusive;

impl LegIterator for Base {
    type PositionIter = Map<StepBy<Rev<RangeInclusive<u64>>>, fn(u64) -> Position>;

    fn get_range(last_position: Position, limit: Position) -> RangeInclusive<Position> {
        limit..=last_position
    }

    fn step_iter<const BITS: u16>(range: RangeInclusive<Position>) -> Self::PositionIter {
        let extracted_range = range.extract_range::<BITS>();
        (extracted_range.start().inner..=extracted_range.end().inner)
            .rev()
            .step_by(BitsLayout::<BITS>::step_interval())
            .map(Position::new)
    }
}
