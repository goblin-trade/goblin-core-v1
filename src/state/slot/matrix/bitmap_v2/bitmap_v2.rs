use core::marker::PhantomData;
use core::ops::RangeInclusive;
use std::u64;

use crate::axis::leg::leg_matcher::LegMatcher;
use crate::quantities::index::Index;

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
    pub fn index_active(&self, index: I) -> bool {
        // map to row (byte) and column (bit)
        // Add another bound such that .inner is only of type u8
    }

    pub fn active_iterator<In>(clamped_range: RangeInclusive<I>) -> impl Iterator<Item = I>
    where
        In: LegMatcher,
    {
        // TODO add iterator type
        // We need to call the iterator generation function generically
        // I::get_iter::<In>(clamped_range).filter(|index| {
        //     // TODO
        // })
    }
    // pub type PrevComposite = (<I::Prev as Index>::Prev, I::Prev);

    // pub fn clamp_range(
    //     range: RangeInclusive<I>,
    //     outer_range: RangeInclusive<OuterIndex<I>>,
    //     current_outer: OuterIndex<I>,
    // ) {
    // }
}
