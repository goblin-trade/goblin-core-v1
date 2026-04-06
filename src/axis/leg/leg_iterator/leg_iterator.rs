use crate::{axis::leg::leg_coordinates::LegCoordinates, quantities::Position};
use core::ops::RangeInclusive;

/// Iterators of coordinates
///
/// Step trait is unstable. We are forced to declare dedicated types
/// and getter functions for each variant.
pub trait LegIterator: LegCoordinates {
    type PositionIter: Iterator<Item = Position>;

    fn position_iter(range: RangeInclusive<Position>) -> Self::PositionIter;

    // type OuterBitmapIndexIter: Iterator<Item = OuterBitmapIndex>;
    // type OuterPosIter: Iterator<Item = OuterPos>;
    // type InnerPosIter: Iterator<Item = InnerPos>;

    // type RowIter: Iterator<Item = Row>;

    // fn outer_bitmap_index_iter(
    //     range: RangeInclusive<OuterBitmapIndex>,
    // ) -> Self::OuterBitmapIndexIter;

    // fn outer_pos_iter(range: RangeInclusive<OuterPos>) -> Self::OuterPosIter;

    // fn inner_pos_iter(range: RangeInclusive<InnerPos>) -> Self::InnerPosIter;

    // fn row_iter(range: RangeInclusive<Row>) -> Self::RowIter;
}
