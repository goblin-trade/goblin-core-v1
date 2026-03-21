use crate::{
    axis::leg::leg_coordinates::LegCoordinates,
    matching::bitmap::{
        inner_pos::InnerPos, outer_bitmap_index::OuterBitmapIndex, outer_pos::OuterPos, row::Row,
    },
};
use core::ops::RangeInclusive;

/// Iterators of coordinates
///
/// Step trait is unstable. We are forced to declare dedicated types
/// and getter functions for each variant.
pub trait LegIterator: LegCoordinates {
    type OuterBitmapIndexIter: Iterator<Item = OuterBitmapIndex>;
    type OuterPosIter: Iterator<Item = OuterPos>;
    type InnerPosIter: Iterator<Item = InnerPos>;

    type RowIter: Iterator<Item = Row>;

    fn outer_bitmap_index_iter(
        range: RangeInclusive<OuterBitmapIndex>,
    ) -> Self::OuterBitmapIndexIter;

    fn outer_pos_iter(range: RangeInclusive<OuterPos>) -> Self::OuterPosIter;

    fn inner_pos_iter(range: RangeInclusive<InnerPos>) -> Self::InnerPosIter;

    fn row_iter(range: RangeInclusive<Row>) -> Self::RowIter;
}
