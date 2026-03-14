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
    type OuterBitmapIndexIter: Iterator<Item = OuterBitmapIndex<Self>>;
    type OuterPosIter: Iterator<Item = OuterPos<Self>>;
    type InnerPosIter: Iterator<Item = InnerPos<Self>>;

    type RowIter: Iterator<Item = Row<Self>>;

    fn outer_bitmap_index_iter(
        range: RangeInclusive<OuterBitmapIndex<Self>>,
    ) -> Self::OuterBitmapIndexIter;

    fn outer_pos_iter(range: RangeInclusive<OuterPos<Self>>) -> Self::OuterPosIter;

    fn inner_pos_iter(range: RangeInclusive<InnerPos<Self>>) -> Self::InnerPosIter;

    fn row_iter(range: RangeInclusive<Row<Self>>) -> Self::RowIter;
}
