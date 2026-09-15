use crate::{
    axis::leg::{LegIterator, Quote},
    quantities::{BitsLayout, FullPos},
};
use core::range::RangeInclusive;

impl LegIterator for Quote {
    fn get_range(last_position: FullPos, limit: FullPos) -> RangeInclusive<FullPos> {
        RangeInclusive {
            start: last_position,
            last: limit,
        }
    }

    fn step_iter<const BITS: u16>(range: RangeInclusive<FullPos>) -> impl Iterator<Item = FullPos> {
        RangeInclusive {
            start: range.start.inner,
            last: range.last.inner,
        }
        .iter()
        .step_by(BitsLayout::<BITS>::step_interval())
        .map(FullPos::new)
    }
}
