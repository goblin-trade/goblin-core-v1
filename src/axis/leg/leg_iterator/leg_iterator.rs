use crate::{
    axis::leg::leg_quantities::LegQuantities,
    matching::bitmap::{
        inner_pos::InnerPos, outer_bitmap_index::OuterBitmapIndex, outer_pos::OuterPos,
        range::Range, row::Row,
    },
};

/// Iterators of coordinates
///
/// Step trait is unstable. We are forced to declare dedicated types
/// and getter functions for each variant.
pub trait LegIterator: LegQuantities {
    type OuterBitmapIndexIter: Iterator<Item = OuterBitmapIndex<Self>>;
    type OuterPosIter: Iterator<Item = OuterPos<Self>>;
    type InnerPosIter: Iterator<Item = InnerPos<Self>>;

    type RowIter: Iterator<Item = Row<Self>>;

    fn outer_bitmap_index_iter(range: OuterBitmapIndex<Self>) -> Self::OuterBitmapIndexIter;

    fn outer_pos_iter(start: OuterPos<Self>) -> Self::OuterPosIter;

    fn inner_pos_iter(start: InnerPos<Self>) -> Self::InnerPosIter;

    fn row_iter(start: Row<Self>) -> Self::RowIter;
}
