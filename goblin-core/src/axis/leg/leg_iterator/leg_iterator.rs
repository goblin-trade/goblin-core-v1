use crate::{
    axis::leg::LegCoordinates,
    quantities::{
        INNER_POS, InnerPos, OUTER_BITMAP_INDEX, OUTER_POS, OuterBitmapIndex, OuterPos, Position,
        PositionRange,
    },
};
use core::range::RangeInclusive;

/// Iterators of coordinates
///
/// Step trait is unstable. We are forced to declare dedicated types
/// and getter functions for each variant.
pub trait LegIterator: LegCoordinates {
    /// Normalize last and limit positions into a RangeInclusive struct.
    ///
    /// In RangeInclusive, start is always the smaller value.
    fn get_range(last_position: Position, limit: Position) -> RangeInclusive<Position>;

    fn step_iter<const BITS: u16>(
        range: RangeInclusive<Position>,
    ) -> impl Iterator<Item = Position>;

    fn outer_bitmap_index_iter(
        range: RangeInclusive<Position>,
    ) -> impl Iterator<Item = OuterBitmapIndex> {
        let extracted_range = range.extract_range::<OUTER_BITMAP_INDEX>();
        Self::step_iter::<OUTER_BITMAP_INDEX>(extracted_range)
            .map(OuterBitmapIndex::from)
    }

    fn outer_pos_iter(
        range: RangeInclusive<Position>,
        current: Position,
    ) -> impl Iterator<Item = OuterPos> {
        let clamped_range = range.clamp_range::<Self, OUTER_POS>(current);
        Self::step_iter::<OUTER_POS>(clamped_range).map(OuterPos::from)
    }

    fn inner_pos_iter(
        range: RangeInclusive<Position>,
        current: Position,
    ) -> impl Iterator<Item = InnerPos> {
        let clamped_range = range.clamp_range::<Self, INNER_POS>(current);
        Self::step_iter::<INNER_POS>(clamped_range).map(InnerPos::from)
    }
}
