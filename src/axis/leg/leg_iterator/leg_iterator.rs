use crate::{
    axis::leg::leg_quantities::LegQuantities,
    matching::bitmap::{
        compact_coordinates::CompactCoordinates, outer_bitmap_index::OuterBitmapIndex,
        outer_pos::OuterPos, row::Row,
    },
};

/// Iterators of coordinates
///
/// Step trait is unstable. We are forced to declare dedicated types
/// and getter functions for each variant.
pub trait LegIterator: LegQuantities {
    type OuterBitmapIndexIter: Iterator<Item = OuterBitmapIndex<Self>>;
    type OuterPosIter: Iterator<Item = OuterPos<Self>>;
    type RowIter: Iterator<Item = Row<Self>>;
    type CoordinatesIter: Iterator<Item = CompactCoordinates<Self>>;

    fn outer_bitmap_index_iter(item: OuterBitmapIndex<Self>) -> Self::OuterBitmapIndexIter;

    fn outer_pos_iter(item: OuterPos<Self>) -> Self::OuterPosIter;

    fn row_iter(item: Row<Self>) -> Self::RowIter;

    fn coordinates_iter(item: CompactCoordinates<Self>) -> Self::CoordinatesIter;
}
