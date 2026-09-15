use crate::{
    axis::leg::{Base, LegIterator},
    quantities::{BitsLayout, Position},
};
use core::range::RangeInclusive;

impl LegIterator for Base {
    fn get_range(last_position: Position, limit: Position) -> RangeInclusive<Position> {
        RangeInclusive {
            start: limit,
            last: last_position,
        }
    }

    fn step_iter<const BITS: u16>(
        range: RangeInclusive<Position>,
    ) -> impl Iterator<Item = Position> {
        RangeInclusive {
            start: range.start.inner,
            last: range.last.inner,
        }
        .iter()
        .rev()
        .step_by(BitsLayout::<BITS>::step_interval())
        .map(Position::new)
    }
}
