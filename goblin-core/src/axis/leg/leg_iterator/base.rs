use crate::{
    axis::leg::{Base, LegIterator},
    quantities::{BitsLayout, PositionV2},
};
use core::range::RangeInclusive;

impl LegIterator for Base {
    fn get_range(last_position: PositionV2, limit: PositionV2) -> RangeInclusive<PositionV2> {
        RangeInclusive {
            start: limit,
            last: last_position,
        }
    }

    fn step_iter<const BITS: u16>(
        range: RangeInclusive<PositionV2>,
    ) -> impl Iterator<Item = PositionV2> {
        RangeInclusive {
            start: range.start.inner,
            last: range.last.inner,
        }
        .iter()
        .rev()
        .step_by(BitsLayout::<BITS>::step_interval())
        .map(PositionV2::new)
    }
}
