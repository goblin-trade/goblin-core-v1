use crate::axis::leg::{leg_iterator::LegIterator, Base};
use crate::quantities::bits_layout::BitsLayout;
use crate::quantities::{InnerPos, OuterBitmapIndex, OuterPos, Position};
use core::iter::{Map, Rev, StepBy};
use core::ops::RangeInclusive;

impl LegIterator for Base {
    // type PositionIter = Map<StepBy<Rev<RangeInclusive<u64>>>, fn(u64) -> Position>;

    type OuterBitmapIndexIter = Map<Rev<RangeInclusive<u64>>, fn(u64) -> OuterBitmapIndex>;
    type OuterPosIter = Map<Rev<RangeInclusive<u8>>, fn(u8) -> OuterPos>;
    type InnerPosIter = Map<Rev<RangeInclusive<u8>>, fn(u8) -> InnerPos>;

    fn get_range(last_position: Position, limit: Position) -> RangeInclusive<Position> {
        limit..=last_position
    }

    fn outer_bitmap_index_iter(
        range: RangeInclusive<OuterBitmapIndex>,
    ) -> Self::OuterBitmapIndexIter {
        (range.start().inner..=range.end().inner)
            .rev()
            .map(OuterBitmapIndex::new)
    }

    fn outer_pos_iter(range: RangeInclusive<OuterPos>) -> Self::OuterPosIter {
        (range.start().inner..=range.end().inner)
            .rev()
            .map(OuterPos::new)
    }

    fn inner_pos_iter(range: RangeInclusive<InnerPos>) -> Self::InnerPosIter {
        (range.start().inner..=range.end().inner)
            .rev()
            .map(InnerPos::new)
    }

    // fn step_iter<const BITS: u16>(range: RangeInclusive<Position>) -> Self::PositionIter {
    //     (range.start().inner..=range.end().inner)
    //         .rev()
    //         .step_by(BitsLayout::<BITS>::step_interval())
    //         .map(Position::new)
    // }
}
