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
    }

    // TODO need a previous_bitmap generic
    // - For B0, this will be ()
    // - For B1, this will be B0
    //
    // We need a trait to break recursion and to integrate () and Bitmap
    // We can't use generic.
    //
    // trait PreviousBitmap
    pub fn get_active_bitmap(previous_index: <BitmapIndexV2<BITS> as OrderedIndex>::Prev) {
        // First think in if-else style
        //
        // if previous_index == () prevous_bitmap is also ()
        //   Read bitmap state and filter for sentinel and 0 bytes
        //
        // Else use previous_index to lookup in previous_bitmap
        //
        // Trait is inevitable since these 2 have different branches.
        // Why not just have separate impls for Outer and Inner bitmap?
        //
    }
}
