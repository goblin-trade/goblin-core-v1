use crate::{
    axis::leg::leg_coordinates::LegCoordinates,
    quantities::{Position, PositionRange, OUTER_BITMAP_INDEX_V2, OUTER_POS_V2},
};
use core::ops::RangeInclusive;
use core::u16;

/// Iterators of coordinates
///
/// Step trait is unstable. We are forced to declare dedicated types
/// and getter functions for each variant.
pub trait LegIterator: LegCoordinates {
    type PositionIter: Iterator<Item = Position>;

    fn step_iter<const BITS: u16>(range: RangeInclusive<Position>) -> Self::PositionIter;

    fn outer_bitmap_index_iter(range: RangeInclusive<Position>) -> Self::PositionIter {
        let extracted_range = range.extract_range::<OUTER_BITMAP_INDEX_V2>();
        Self::step_iter::<OUTER_BITMAP_INDEX_V2>(extracted_range)
    }

    fn outer_pos_iter(range: RangeInclusive<Position>, current: Position) -> Self::PositionIter {
        let clamped_range = range.clamp_range::<Self, OUTER_POS_V2>(current);
        Self::step_iter::<OUTER_POS_V2>(clamped_range)
    }

    fn inner_pos_iter(range: RangeInclusive<Position>, current: Position) -> Self::PositionIter;
}
