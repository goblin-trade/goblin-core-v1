use core::range::RangeInclusive;

use crate::{
    axis::leg::LegCoordinates,
    quantities::{FullPos, FullPosition, InnerVal, Position},
};

pub trait PositionRange: Sized {
    fn map_range<A>(&self, f: impl Fn(FullPos) -> A) -> RangeInclusive<A>;

    fn extract_range<const BITS: u16>(&self) -> Self;

    fn cast_range<K, const BITS: u16>(&self) -> RangeInclusive<Position<K, BITS>>
    where
        K: InnerVal;

    fn clamp_range<In, const BITS: u16>(&self, position: FullPos) -> RangeInclusive<FullPos>
    where
        In: LegCoordinates;
}

impl PositionRange for RangeInclusive<FullPos> {
    fn map_range<A>(&self, f: impl Fn(FullPos) -> A) -> RangeInclusive<A> {
        RangeInclusive {
            start: f(self.start),
            last: f(self.last),
        }
    }

    fn extract_range<const BITS: u16>(&self) -> Self {
        self.map_range(|i| i.extract::<BITS>())
    }

    fn cast_range<K, const BITS: u16>(&self) -> RangeInclusive<Position<K, BITS>>
    where
        K: InnerVal,
    {
        self.map_range(|pos| pos.extract_and_convert())
    }

    fn clamp_range<In, const BITS: u16>(&self, position: FullPos) -> RangeInclusive<FullPos>
    where
        In: LegCoordinates,
    {
        let compliment = position.complement::<BITS>();
        let start = if compliment == self.start.complement::<BITS>() {
            position.extract::<BITS>()
        } else {
            In::start::<BITS>()
        };
        let last = if compliment == self.last.complement::<BITS>() {
            position.extract::<BITS>()
        } else {
            In::end::<BITS>()
        };

        RangeInclusive { start, last }
    }
}
