use crate::{axis::leg::leg_coordinates::LegCoordinates, quantities::Position};
use core::ops::RangeInclusive;

/// Iterators of coordinates
///
/// Step trait is unstable. We are forced to declare dedicated types
/// and getter functions for each variant.
pub trait LegIterator: LegCoordinates {
    type PositionIter: Iterator<Item = Position>;
    // type RowIter: Iterator<Item = Row>;
    // fn row_iter(range: RangeInclusive<Row>) -> Self::RowIter;

    fn outer_bitmap_index_iter(range: RangeInclusive<Position>) -> Self::PositionIter;

    fn outer_pos_iter(range: RangeInclusive<Position>, current: Position) -> Self::PositionIter;

    fn inner_pos_iter(range: RangeInclusive<Position>, current: Position) -> Self::PositionIter;
}
