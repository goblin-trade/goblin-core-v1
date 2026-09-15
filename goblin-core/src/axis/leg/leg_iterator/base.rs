use crate::{
    axis::leg::{Base, LegIterator},
    quantities::{BitsLayout, FullPos},
};
use core::range::RangeInclusive;

impl LegIterator for Base {
    fn get_range(last_position: FullPos, limit: FullPos) -> RangeInclusive<FullPos> {
        RangeInclusive {
            start: limit,
            last: last_position,
        }
    }

    fn step_iter<const BITS: u16>(range: RangeInclusive<FullPos>) -> impl Iterator<Item = FullPos> {
        RangeInclusive {
            start: range.start.inner,
            last: range.last.inner,
        }
        .iter()
        .rev()
        .step_by(BitsLayout::<BITS>::step_interval())
        .map(FullPos::new)
    }
}
