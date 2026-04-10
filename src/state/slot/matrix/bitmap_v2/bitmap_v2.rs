use crate::axis::leg::leg_matcher::LegMatcher;
use crate::state::bitmap_v2::bitmap_index_v2::BitmapIndexV2;
use crate::state::bitmap_v2::ordered_index::OrderedIndex;
use core::ops::RangeInclusive;

pub struct BitmapV2<const BITS: usize> {
    pub inner: [u8; 32],
}

impl<const BITS: usize> BitmapV2<BITS>
where
    BitmapIndexV2<BITS>: OrderedIndex,
{
    pub fn index_active(&self, index: BitmapIndexV2<BITS>) -> bool {
        let byte = self.inner[index.byte_index()];
        let mask = 1 << index.bit_index();

        (byte & mask) != 0
    }

    pub fn active_iterator<In>(
        self,
        clamped_range: RangeInclusive<BitmapIndexV2<BITS>>,
    ) -> impl Iterator<Item = BitmapIndexV2<BITS>>
    where
        In: LegMatcher,
    {
        BitmapIndexV2::<BITS>::get_iter::<In>(clamped_range)
            .filter(move |index| self.index_active(*index))
        // I::get_iter::<In>(clamped_range).filter(move |index| self.index_active(*index))
    }
}

// Alt design to get rid of BitmapIndex trait
// Instead of I: BitmapIndexV2, use Derived position directly with K = 8 and rest as generics
//
// inside the impl block, add a restriction bound OrderedIndex
// pub struct BitmapV2<I>
// where
//     I: BitmapIndexV2,
//     I::Prev: OrderedIndex,
// {
//     pub inner: [u8; 32],
//     _marker: PhantomData<I>,
// }

// impl<I> BitmapV2<I>
// where
//     I: BitmapIndexV2,
//     I::Prev: OrderedIndex,
// {
//     pub fn index_active(&self, index: I) -> bool {
//         let byte = self.inner[index.byte_index()];
//         let mask = 1 << index.bit_index();

//         (byte & mask) != 0
//     }

//     pub fn active_iterator<In>(self, clamped_range: RangeInclusive<I>) -> impl Iterator<Item = I>
//     where
//         In: LegMatcher,
//     {
//         I::get_iter::<In>(clamped_range).filter(move |index| self.index_active(*index))
//     }
// }
