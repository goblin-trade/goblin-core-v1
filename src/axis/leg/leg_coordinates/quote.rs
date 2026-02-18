use crate::{
    axis::leg::{leg_coordinates::LegCoordinates, Quote},
    matching::bitmap::outer_bitmap_index::OuterBitmapIndex,
};

impl LegCoordinates for Quote {
    fn get_iter(item: OuterBitmapIndex<Self>) -> impl Iterator<Item = OuterBitmapIndex<Self>> {
        (item.inner..=u64::MAX).map(OuterBitmapIndex::<Self>::new)
    }
}
