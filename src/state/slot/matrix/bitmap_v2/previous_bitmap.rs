use crate::state::bitmap_v2::{
    bitmap_index_v2::BitmapIndexV2, ordered_index::OrderedIndex, BitmapV2,
};

// We need pair- ((), ()) and (index, bitmap)
//
// Fix- define PreviousIndex: Clone + Copy on () and DerivedPosition
// Then use it in OrderedIndex
//

pub trait PreviousBitmap {}

impl PreviousBitmap for () {}

impl<const BITS: usize> PreviousBitmap for BitmapV2<BITS> where BitmapIndexV2<BITS>: OrderedIndex {}
