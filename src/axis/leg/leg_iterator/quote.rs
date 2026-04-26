use crate::{
    axis::leg::{leg_iterator::LegIterator, Quote},
    quantities::{bits_layout::BitsLayout, Position, PositionRange, INNER_POS_V2},
};
use core::iter::{Map, StepBy};
use core::ops::RangeInclusive;

impl LegIterator for Quote {
    type PositionIter = Map<StepBy<RangeInclusive<u64>>, fn(u64) -> Position>;

    fn step_iter<const BITS: u16>(range: RangeInclusive<Position>) -> Self::PositionIter {
        (range.start().inner..=range.end().inner)
            .step_by(BitsLayout::<BITS>::step_interval())
            .map(Position::new)
    }

    fn inner_pos_iter(range: RangeInclusive<Position>, current: Position) -> Self::PositionIter {
        let clamped_range = range.clamp_range::<Self, INNER_POS_V2>(current);
        Self::step_iter::<INNER_POS_V2>(clamped_range)
    }
}
