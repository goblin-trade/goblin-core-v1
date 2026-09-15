use crate::{
    axis::leg::{LegIterator, Quote},
    quantities::{BitsLayout, PositionV2},
};
use core::range::RangeInclusive;

impl LegIterator for Quote {
    fn get_range(last_position: PositionV2, limit: PositionV2) -> RangeInclusive<PositionV2> {
        RangeInclusive {
            start: last_position,
            last: limit,
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
        .step_by(BitsLayout::<BITS>::step_interval())
        .map(PositionV2::new)
    }
}
