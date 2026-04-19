use crate::{
    axis::leg::leg_matcher::LegMatcher,
    quantities::{DerivedPosition, Position},
};
use core::ops::RangeInclusive;

pub trait PositionRange: Sized {
    fn map_range<A>(&self, f: impl Fn(Position) -> A) -> RangeInclusive<A>;

    fn extract_range<const BITS: u16>(&self) -> Self;

    // fn effective_range<In, const BITS: u16>(
    //     &self,
    //     position: Position,
    // ) -> RangeInclusive<DerivedPosition<u8, BITS>>
    // where
    //     In: LegMatcher;

    fn effective_range_v2<In, const BITS: u16>(
        &self,
        position: Position,
    ) -> RangeInclusive<Position>
    where
        In: LegMatcher;
}

impl PositionRange for RangeInclusive<Position> {
    fn map_range<A>(&self, f: impl Fn(Position) -> A) -> RangeInclusive<A> {
        f(*self.start())..=f(*self.end())
    }

    fn extract_range<const BITS: u16>(&self) -> Self {
        self.map_range(|i| i.extract::<BITS>())
    }

    // fn effective_range<In, const BITS: u16>(
    //     &self,
    //     position: Position,
    // ) -> RangeInclusive<DerivedPosition<u8, BITS>>
    // where
    //     In: LegMatcher,
    // {
    //     let compliment = position.complement::<BITS>();
    //     let start = if compliment == self.start().complement::<BITS>() {
    //         position.into()
    //     } else {
    //         In::start()
    //     };
    //     let end = if compliment == self.end().complement::<BITS>() {
    //         position.into()
    //     } else {
    //         In::end()
    //     };

    //     start..=end
    // }

    fn effective_range_v2<In, const BITS: u16>(
        &self,
        position: Position,
    ) -> RangeInclusive<Position>
    where
        In: LegMatcher,
    {
        let compliment = position.complement::<BITS>();
        let start = if compliment == self.start().complement::<BITS>() {
            position
        } else {
            In::start_v2::<BITS>()
        };
        let end = if compliment == self.end().complement::<BITS>() {
            position
        } else {
            In::end_v2::<BITS>()
        };

        start..=end
    }
}
