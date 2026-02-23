use crate::{
    axis::leg::leg_quantities::LegQuantities,
    matching::bitmap::{
        compact_coordinates::CompactCoordinates, outer_bitmap_index::OuterBitmapIndex,
        outer_pos::OuterPos, row::Row, Coordinate,
    },
};

pub trait LegIterator: LegQuantities {
    type OuterBitmapIndexIter: Iterator<Item = OuterBitmapIndex<Self>>;

    fn outer_bitmap_index_iter(item: OuterBitmapIndex<Self>) -> Self::OuterBitmapIndexIter;

    fn outer_pos_iter(item: OuterPos<Self>) -> impl Iterator<Item = OuterPos<Self>>;

    fn row_iter(item: Row<Self>) -> impl Iterator<Item = Row<Self>>;

    fn coordinates_iter(
        item: CompactCoordinates<Self>,
    ) -> impl Iterator<Item = CompactCoordinates<Self>>;
}
