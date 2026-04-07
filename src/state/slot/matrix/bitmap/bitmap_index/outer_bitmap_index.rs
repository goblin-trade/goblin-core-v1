use core::ops::RangeInclusive;

use crate::{
    axis::leg::leg_iterator::LegIterator, quantities::OuterBitmapIndexV2,
    state::bitmap::bitmap_index::BitmapIndex,
};

impl BitmapIndex for OuterBitmapIndexV2 {
    fn build_iterator<In>(range: RangeInclusive<Self>) -> impl Iterator<Item = Self>
    where
        In: LegIterator,
    {
        In::outer_bitmap_index_iter(range)
    }
}
