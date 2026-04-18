use core::ops::RangeInclusive;

use crate::{
    axis::leg::leg_matcher::LegMatcher,
    quantities::{DerivedPosition, Position},
};

pub trait PositionRange: Sized {
    fn map_range<A>(&self, f: impl Fn(Position) -> A) -> RangeInclusive<A>;

    fn compliment_range<const BITS: u16>(&self) -> Self;

    fn effective_range<const BITS: u16, In>(
        &self,
        position: Position,
    ) -> RangeInclusive<DerivedPosition<u8, BITS>>
    where
        In: LegMatcher;
}

impl PositionRange for RangeInclusive<Position> {
    fn map_range<A>(&self, f: impl Fn(Position) -> A) -> RangeInclusive<A> {
        f(*self.start())..=f(*self.end())
    }

    fn compliment_range<const BITS: u16>(&self) -> Self {
        self.map_range(|p| p.complement::<BITS>())
    }

    fn effective_range<const BITS: u16, In>(
        &self,
        position: Position,
    ) -> RangeInclusive<DerivedPosition<u8, BITS>>
    where
        In: LegMatcher,
    {
        let compliment = position.complement::<BITS>();
        let start = if compliment == self.start().complement::<BITS>() {
            position.into()
        } else {
            In::start()
        };
        let end = if compliment == self.end().complement::<BITS>() {
            position.into()
        } else {
            In::end()
        };

        start..=end
    }
}
