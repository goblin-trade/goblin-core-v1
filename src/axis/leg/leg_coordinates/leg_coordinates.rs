use crate::{
    axis::leg::leg_quantities::LegQuantities,
    matching::bitmap::outer_bitmap_index::OuterBitmapIndex,
};

pub trait LegCoordinates: LegQuantities {
    fn next_outer_bitmap_index(item: OuterBitmapIndex<Self>) -> Option<OuterBitmapIndex<Self>>;
}
