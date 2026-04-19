use crate::{
    axis::leg::{leg_iterator::LegIterator, Quote},
    quantities::{
        bits_layout::BitsLayout, Position, PositionRange, INNER_POS_V2, OUTER_BITMAP_INDEX_V2,
        OUTER_POS_V2,
    },
};
use core::iter::{Map, StepBy};
use core::ops::RangeInclusive;

impl LegIterator for Quote {
    type PositionIter = Map<StepBy<RangeInclusive<u64>>, fn(u64) -> Position>;

    fn outer_bitmap_index_iter(range: RangeInclusive<Position>) -> Self::PositionIter {
        let extracted_range = range.extract_range::<OUTER_BITMAP_INDEX_V2>();
        (extracted_range.start().inner..=extracted_range.end().inner)
            .step_by(BitsLayout::<OUTER_BITMAP_INDEX_V2>::step_interval())
            .map(Position::new)
    }

    fn outer_pos_iter(range: RangeInclusive<Position>, current: Position) -> Self::PositionIter {
        let extracted_range = range.effective_range::<Self, OUTER_POS_V2>(current);
        (extracted_range.start().inner..=extracted_range.end().inner)
            .step_by(BitsLayout::<OUTER_POS_V2>::step_interval())
            .map(Position::new)
    }

    fn inner_pos_iter(range: RangeInclusive<Position>, current: Position) -> Self::PositionIter {
        let extracted_range = range.effective_range::<Self, INNER_POS_V2>(current);
        (extracted_range.start().inner..=extracted_range.end().inner)
            .step_by(BitsLayout::<INNER_POS_V2>::step_interval())
            .map(Position::new)
    }
}
