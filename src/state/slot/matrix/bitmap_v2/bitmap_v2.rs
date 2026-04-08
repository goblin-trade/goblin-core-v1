use core::marker::PhantomData;
use core::ops::RangeInclusive;

use crate::state::bitmap_v2::index::Index;

pub struct BitmapV2<I>
where
    I: Index,
    I::Outer: Index,
{
    pub inner: [u8; 32],
    _marker: PhantomData<I>,
}

impl<I> BitmapV2<I>
where
    I: Index,
    I::Outer: Index,
{
    // pub type PrevComposite = (<I::Prev as Index>::Prev, I::Prev);

    // pub fn clamp_range(
    //     range: RangeInclusive<I>,
    //     outer_range: RangeInclusive<OuterIndex<I>>,
    //     current_outer: OuterIndex<I>,
    // ) {
    // }
}
