use crate::{
    axis::leg::{leg_iterator::LegIterator, Quote},
    quantities::{bits_layout::BitsLayout, InnerPos, OuterBitmapIndex, OuterPos, Pos2, Position},
};
use core::iter::{Map, StepBy};
use core::ops::RangeInclusive;

impl LegIterator for Quote {
    type PositionIter = Map<StepBy<RangeInclusive<u64>>, fn(u64) -> Position>;

    type OuterBitmapIndexIter = Map<StepBy<RangeInclusive<u64>>, fn(u64) -> OuterBitmapIndex>;
    type OuterPosIter = Map<RangeInclusive<u8>, fn(u8) -> OuterPos>;
    type InnerPosIter = Map<RangeInclusive<u8>, fn(u8) -> InnerPos>;

    fn get_range(last_position: Pos2, limit: Pos2) -> RangeInclusive<Pos2> {
        last_position..=limit
    }

    // fn outer_bitmap_index_iter(
    //     range: RangeInclusive<OuterBitmapIndex>,
    // ) -> Self::OuterBitmapIndexIter {
    //     (range.start().inner..=range.end().inner).map(OuterBitmapIndex::new)
    // }

    // fn outer_pos_iter(range: RangeInclusive<OuterPos>) -> Self::OuterPosIter {
    //     (range.start().inner..=range.end().inner).map(OuterPos::new)
    // }

    // fn inner_pos_iter(range: RangeInclusive<InnerPos>) -> Self::InnerPosIter {
    //     (range.start().inner..=range.end().inner).map(InnerPos::new)
    // }

    fn step_iter<const BITS: u16>(range: RangeInclusive<Position>) -> Self::PositionIter {
        (range.start().inner..=range.end().inner)
            .step_by(BitsLayout::<BITS>::step_interval())
            .map(Position::new)
    }
}
