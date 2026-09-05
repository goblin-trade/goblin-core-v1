use crate::{
    axis::leg::{Base, LegIterator},
    quantities::{BitsLayout, Position},
};
use core::ops::RangeInclusive;

impl LegIterator for Base {
    fn get_range(last_position: Position, limit: Position) -> RangeInclusive<Position> {
        limit..=last_position
    }

    fn step_iter<const BITS: u16>(
        range: RangeInclusive<Position>,
    ) -> impl Iterator<Item = Position> {
        (range.start().inner..=range.end().inner)
            .rev()
            .step_by(BitsLayout::<BITS>::step_interval())
            .map(Position::new)
    }
}
