use crate::{
    axis::leg::{leg_coordinates::LegCoordinates, Base},
    matching::bitmap::{outer_bitmap_index::OuterBitmapIndex, outer_pos::OuterPos, row::Row},
};

impl LegCoordinates for Base {
    fn outer_bitmap_index_iter(
        item: OuterBitmapIndex<Self>,
    ) -> impl Iterator<Item = OuterBitmapIndex<Self>> {
        (0..=item.inner).rev().map(OuterBitmapIndex::<Self>::new)
    }

    fn outer_pos_iter(item: OuterPos<Self>) -> impl Iterator<Item = OuterPos<Self>> {
        (0..=item.inner).rev().map(OuterPos::<Self>::new)
    }

    fn row_iter(item: Row<Self>) -> impl Iterator<Item = Row<Self>> {
        (0..=item.inner).rev().map(Row::<Self>::new)
    }
}
