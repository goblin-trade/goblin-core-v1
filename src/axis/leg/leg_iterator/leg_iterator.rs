use crate::{
    axis::leg::leg_coordinates::LegCoordinates,
    quantities::{InnerPosV2, OuterBitmapIndexV2, OuterPosV2},
};
use core::ops::RangeInclusive;

/// Iterators of coordinates
///
/// Step trait is unstable. We are forced to declare dedicated types
/// and getter functions for each variant.
pub trait LegIterator: LegCoordinates {
    // type PositionIter: Iterator<Item = Position>;

    // fn position_iter(range: RangeInclusive<Position>) -> Self::PositionIter;

    type OuterBitmapIndexIter: Iterator<Item = OuterBitmapIndexV2>;
    type OuterPosIter: Iterator<Item = OuterPosV2>;
    type InnerPosIter: Iterator<Item = InnerPosV2>;

    // type RowIter: Iterator<Item = Row>;

    fn outer_bitmap_index_iter(
        range: RangeInclusive<OuterBitmapIndexV2>,
    ) -> Self::OuterBitmapIndexIter;

    fn outer_pos_iter(range: RangeInclusive<OuterPosV2>) -> Self::OuterPosIter;

    fn inner_pos_iter(range: RangeInclusive<InnerPosV2>) -> Self::InnerPosIter;

    // fn row_iter(range: RangeInclusive<Row>) -> Self::RowIter;
}
