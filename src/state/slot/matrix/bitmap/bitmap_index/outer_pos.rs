use core::ops::RangeInclusive;

use crate::{
    axis::leg::leg_iterator::LegIterator, quantities::OuterPosV2,
    state::bitmap::bitmap_index::BitmapIndex,
};

// Collision
//
// 1. Inside InnerBitmapIterator we only iterate through OuterPos, not OuterBitmapIndex which
// remains constant.
// 2. However BitmapIndex trait was to be defined on (OuterBitmapIndex, OuterPos)
//
// If we update the trait bound, we cannot perform equality check on (OuterBitmapIndex, OuterPos)
// for resetting InnerPos
//
// Hack- define it for (OuterBitmapIndex, OuterPos). Internally use In::outer_pos_iter(range),
// while prepending the constant OuterBitmapIndex
//
// Problem- if we use composite index, there are 2 values of OuterBitmapIndex. We have an invalid
// state where the two OuterBitmapIndex are different
impl BitmapIndex for OuterPosV2 {
    fn build_iterator<In>(range: RangeInclusive<Self>) -> impl Iterator<Item = Self>
    where
        In: LegIterator,
    {
        In::outer_pos_iter(range)
    }
}
