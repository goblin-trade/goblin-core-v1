use crate::axis::leg::leg_matcher::LegMatcher;
use crate::state::bitmap_v2::bitmap_index_v2::BitmapIndexV2;
use crate::state::bitmap_v2::ordered_index::OrderedIndex;
use core::ops::RangeInclusive;

pub struct BitmapV2<const BIT_OFFSET: usize, const BIT_COUNT: usize> {
    pub inner: [u8; 32],
}

impl<const BIT_OFFSET: usize, const BIT_COUNT: usize> BitmapV2<BIT_OFFSET, BIT_COUNT>
where
    BitmapIndexV2<BIT_OFFSET, BIT_COUNT>: OrderedIndex,
{
    pub fn index_active(&self, index: BitmapIndexV2<BIT_OFFSET, BIT_COUNT>) -> bool {
        let byte = self.inner[index.byte_index()];
        let mask = 1 << index.bit_index();

        (byte & mask) != 0
    }

    pub fn active_iterator<In>(
        self,
        clamped_range: RangeInclusive<BitmapIndexV2<BIT_OFFSET, BIT_COUNT>>,
    ) -> impl Iterator<Item = BitmapIndexV2<BIT_OFFSET, BIT_COUNT>>
    where
        In: LegMatcher,
    {
        BitmapIndexV2::<BIT_OFFSET, BIT_COUNT>::get_iter::<In>(clamped_range)
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
