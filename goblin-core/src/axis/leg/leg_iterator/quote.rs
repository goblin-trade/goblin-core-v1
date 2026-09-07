use crate::{
    axis::leg::{LegIterator, Quote},
    quantities::{BitsLayout, Position},
};
use core::range::RangeInclusive;

impl LegIterator for Quote {
    fn get_range(last_position: Position, limit: Position) -> RangeInclusive<Position> {
        RangeInclusive {
            start: last_position,
            last: limit,
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
        .step_by(BitsLayout::<BITS>::step_interval())
        .map(Position::new)
    }
}
