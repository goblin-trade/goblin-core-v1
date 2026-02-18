use crate::{
    axis::leg::{leg_coordinates::LegCoordinates, Base},
    matching::bitmap::outer_bitmap_index::OuterBitmapIndex,
};

impl LegCoordinates for Base {
    fn next_outer_bitmap_index(item: OuterBitmapIndex<Self>) -> Option<OuterBitmapIndex<Self>> {
        item.inner.checked_sub(1).map(OuterBitmapIndex::new)
    }
}
