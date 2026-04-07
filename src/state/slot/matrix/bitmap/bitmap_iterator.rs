use core::ops::RangeInclusive;

use crate::{
    axis::{
        leg::leg_matcher::LegMatcher, market::market_marker::MarketMarker,
        token::token_marker::TokenMarker,
    },
    quantities::{OuterBitmapIndexV2, Position},
    state::{bitmap::Bitmap, MarketPreimage, SlotKey},
};

pub struct BitmapEntry<P0, P1, BM>
where
    P0: Clone + Copy + PartialEq + PartialOrd,
    P1: Clone + Copy + PartialEq + PartialOrd,
    BM: Bitmap<P0, P1>,
{
    pub bitmap_index: P0,
    pub bitmap: BM,
    pub child_range: RangeInclusive<P1>,
}

pub trait BitmapIterator<P0, P1>: Bitmap<P0, P1>
where
    P0: Clone + Copy + PartialEq + PartialOrd,
    P1: Clone + Copy + PartialEq + PartialOrd,
{
    fn build_iterator<M, B, Q, In>(
        market_key: SlotKey<MarketPreimage<M, B, Q>>,
        range: RangeInclusive<(P0, P1)>,
    ) -> impl Iterator<Item = BitmapEntry<P0, P1, Self>>
    where
        M: MarketMarker,
        B: TokenMarker,
        Q: TokenMarker,
        In: LegMatcher,
    {
        // Problem- this should work generically
        //
        // Add `DerivedPosition` trait bound on P0 and P1?
        //
        // Problem
        // 1. We can't simply pass `range` to generic iterator, It needs to be P0
        //
        // Solution- decompose iterator further. The design will differ from
        // the old iterator, in that the external iterator will not have knowledge
        // about any inner iterator. The two will be composed externally.

        let (start, end) = range.into_inner();

        // TODO define BitmapIndex trait that maps In: LegMarker and RangeInclusive<P0> to iterator
        let outer_range = start.0..=end.0;

        // Declare outer iterator on start.0 and end.0, i.e. on RangeInclusive<P0>
        // 1. OuterBitmap: This is RangeInclusive<OuterBitmap>
        // 2. InnerBitmap: This is RangeInclusive<(OuterBitmap, InnerBitmap)>.
        // We then match iterator position with start and end to update the inner iterator bounds.
        // let index_range = OuterBitmapIndexV2::convert_range(range);

        In::outer_bitmap_index_iter(index_range)
    }
}
