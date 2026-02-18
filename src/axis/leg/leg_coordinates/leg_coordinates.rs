use crate::{
    axis::leg::leg_quantities::LegQuantities,
    matching::bitmap::outer_bitmap_index::OuterBitmapIndex,
};

pub trait LegCoordinates: LegQuantities {
    fn get_iter(item: OuterBitmapIndex<Self>) -> impl Iterator<Item = OuterBitmapIndex<Self>>;
}
