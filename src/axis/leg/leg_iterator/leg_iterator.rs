use crate::quantities::{
    DerivedPosition, InnerPos, OuterBitmapIndex, OuterPos, Pos2, PositionRange,
};
use crate::{
    axis::leg::leg_coordinates::LegCoordinates,
    quantities::{Position, INNER_POS, OUTER_BITMAP_INDEX, OUTER_POS},
};
use core::ops::RangeInclusive;
use core::u16;

/// Iterators of coordinates
///
/// Step trait is unstable. We are forced to declare dedicated types
/// and getter functions for each variant.
pub trait LegIterator: LegCoordinates {
    // type PositionIter: Iterator<Item = Position>;

    // problem- this becomes dirty. we previously had a single step_iter function
    type OuterBitmapIndexIter: Iterator<Item = OuterBitmapIndex>;
    type OuterPosIter: Iterator<Item = OuterPos>;
    type InnerPosIter: Iterator<Item = InnerPos>;

    fn outer_bitmap_index_iter(
        range: RangeInclusive<OuterBitmapIndex>,
    ) -> Self::OuterBitmapIndexIter;

    fn outer_pos_iter(range: RangeInclusive<OuterPos>) -> Self::OuterPosIter;

    fn inner_pos_iter(range: RangeInclusive<InnerPos>) -> Self::InnerPosIter;

    /// Normalize last and limit positions into a RangeInclusive struct.
    ///
    /// In RangeInclusive, start is always the smaller value.
    fn get_range(last_position: Pos2, limit: Pos2) -> RangeInclusive<Pos2>;

    // fn step_iter<const BITS: u16>(range: RangeInclusive<Position>) -> Self::PositionIter;

    // fn outer_bitmap_index_iter(range: RangeInclusive<Position>) -> Self::PositionIter {
    //     let extracted_range = range.extract_range::<OUTER_BITMAP_INDEX>();
    //     Self::step_iter::<OUTER_BITMAP_INDEX>(extracted_range)
    // }

    fn outer_pos_iter(range: RangeInclusive<Position>, current: Position) -> Self::PositionIter {
        let clamped_range = range.clamp_range::<Self, OUTER_POS>(current);
        Self::step_iter::<OUTER_POS>(clamped_range)
    }

    // fn inner_pos_iter(range: RangeInclusive<Position>, current: Position) -> Self::PositionIter {
    //     let clamped_range = range.clamp_range::<Self, INNER_POS>(current);
    //     Self::step_iter::<INNER_POS>(clamped_range)
    // }
}
