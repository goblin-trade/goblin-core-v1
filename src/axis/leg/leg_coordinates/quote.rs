use crate::{
    axis::leg::{leg_coordinates::LegCoordinates, Quote},
    matching::bitmap::outer_bitmap_index::OuterBitmapIndex,
};

impl LegCoordinates for Quote {
    fn next_outer_bitmap_index(item: OuterBitmapIndex<Self>) -> Option<OuterBitmapIndex<Self>> {
        item.inner.checked_add(1).map(OuterBitmapIndex::new)
    }
}
