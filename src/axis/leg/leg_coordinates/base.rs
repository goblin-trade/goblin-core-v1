use crate::{
    axis::leg::{leg_coordinates::LegCoordinates, Base},
    matching::bitmap::outer_bitmap_index::OuterBitmapIndex,
};

impl LegCoordinates for Base {
    fn get_iter(item: OuterBitmapIndex<Self>) -> impl Iterator<Item = OuterBitmapIndex<Self>> {
        (0..=item.inner).rev().map(OuterBitmapIndex::<Self>::new)
    }
}
