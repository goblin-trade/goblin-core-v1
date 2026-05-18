use core::ops::RangeInclusive;

use crate::{
    axis::leg::leg_coordinates::LegCoordinates,
    quantities::{inner_val::InnerVal, DerivedPosition, Pos2, Position},
};

impl From<RangeInclusive<Pos2>> for RangeInclusive<Position> {
    fn from(value: RangeInclusive<Pos2>) -> Self {
        value.start().position()..=value.end().position()
    }
}

pub trait PositionRange: Sized {
    fn map_range<A>(&self, f: impl Fn(Position) -> A) -> RangeInclusive<A>;

    fn extract_range<const BITS: u16>(&self) -> Self;

    fn cast_range<K, const BITS: u16>(&self) -> RangeInclusive<DerivedPosition<K, BITS>>
    where
        K: InnerVal;

    fn clamp_range<In, const BITS: u16>(&self, position: Position) -> RangeInclusive<Position>
    where
        In: LegCoordinates;
}

impl PositionRange for RangeInclusive<Position> {
    fn map_range<A>(&self, f: impl Fn(Position) -> A) -> RangeInclusive<A> {
        f(*self.start())..=f(*self.end())
    }

    fn extract_range<const BITS: u16>(&self) -> Self {
        self.map_range(|i| i.extract::<BITS>())
    }

    fn cast_range<K, const BITS: u16>(&self) -> RangeInclusive<DerivedPosition<K, BITS>>
    where
        K: InnerVal,
    {
        self.map_range(|pos| pos.into())
    }

    fn clamp_range<In, const BITS: u16>(&self, position: Position) -> RangeInclusive<Position>
    where
        In: LegCoordinates,
    {
        let compliment = position.complement::<BITS>();
        let start = if compliment == self.start().complement::<BITS>() {
            position.extract::<BITS>()
        } else {
            In::start::<BITS>()
        };
        let end = if compliment == self.end().complement::<BITS>() {
            position.extract::<BITS>()
        } else {
            In::end::<BITS>()
        };

        start..=end
    }
}
