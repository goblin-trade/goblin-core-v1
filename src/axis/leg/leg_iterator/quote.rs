use crate::{
    axis::leg::{leg_iterator::LegIterator, Quote},
    quantities::{bits_layout::BitsLayout, Position, PositionRange},
};
use core::iter::{Map, StepBy};
use core::ops::RangeInclusive;

impl LegIterator for Quote {
    type PositionIter = Map<StepBy<RangeInclusive<u64>>, fn(u64) -> Position>;

    fn get_range(last_position: Position, limit: Position) -> RangeInclusive<Position> {
        last_position..=limit
    }

    fn step_iter<const BITS: u16>(range: RangeInclusive<Position>) -> Self::PositionIter {
        let extracted_range = range.extract_range::<BITS>();
        (extracted_range.start().inner..=extracted_range.end().inner)
            .step_by(BitsLayout::<BITS>::step_interval())
            .map(Position::new)
    }
}
