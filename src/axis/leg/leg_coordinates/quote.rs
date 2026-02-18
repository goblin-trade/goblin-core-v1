use crate::{
    axis::leg::{leg_coordinates::LegCoordinates, Quote},
    matching::bitmap::{outer_bitmap_index::OuterBitmapIndex, outer_pos::OuterPos, row::Row},
};

impl LegCoordinates for Quote {
    fn outer_bitmap_index_iter(
        item: OuterBitmapIndex<Self>,
    ) -> impl Iterator<Item = OuterBitmapIndex<Self>> {
        (item.inner..=u64::MAX).map(OuterBitmapIndex::<Self>::new)
    }

    fn outer_pos_iter(item: OuterPos<Self>) -> impl Iterator<Item = OuterPos<Self>> {
        (item.inner..=u8::MAX).map(OuterPos::<Self>::new)
    }

    fn row_iter(item: Row<Self>) -> impl Iterator<Item = Row<Self>> {
        (item.inner..=u8::MAX).map(Row::<Self>::new)
    }
}
