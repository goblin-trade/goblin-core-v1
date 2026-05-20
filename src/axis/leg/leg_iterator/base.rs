use crate::axis::leg::{leg_iterator::LegIterator, Base};
use crate::quantities::bits_layout::BitsLayout;
use crate::quantities::{Pos2, Position};
use core::ops::RangeInclusive;

impl LegIterator for Base {
    fn get_range(last_position: Pos2, limit: Pos2) -> RangeInclusive<Pos2> {
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
