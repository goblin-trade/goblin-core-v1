use crate::{
    axis::leg::{leg_iterator::LegIterator, Quote},
    quantities::{bits_layout::BitsLayout, Position},
};
use core::ops::RangeInclusive;

impl LegIterator for Quote {
    fn get_range(last_position: Position, limit: Position) -> RangeInclusive<Position> {
        last_position..=limit
    }

    fn step_iter<const BITS: u16>(
        range: RangeInclusive<Position>,
    ) -> impl Iterator<Item = Position> {
        (range.start().inner..=range.end().inner)
            .step_by(BitsLayout::<BITS>::step_interval())
            .map(Position::new)
    }
}
