use core::marker::PhantomData;
use core::ops::RangeInclusive;

use crate::{
    axis::{
        leg::leg_matcher::LegMatcher, market::market_marker::MarketMarker,
        token::token_marker::TokenMarker,
    },
    quantities::{OuterBitmapIndexV2, Position},
    state::{
        bitmap::{bitmap_index::BitmapIndex, Bitmap},
        MarketPreimage, SlotKey,
    },
};

pub struct BitmapEntry<F, P0, P1, BM>
where
    F: Clone + Copy + PartialEq + PartialOrd,
    P0: BitmapIndex,
    P1: Clone + Copy + PartialEq + PartialOrd,
    BM: Bitmap<F, P0, P1>,
{
    pub bitmap_index: P0,
    pub bitmap: BM,
    pub child_range: RangeInclusive<P1>,
    _marker: PhantomData<F>,
}

pub trait BitmapIterator<F, P0, P1>: Bitmap<F, P0, P1>
where
    F: Clone + Copy + PartialEq + PartialOrd,
    P0: BitmapIndex,
    P1: Clone + Copy + PartialEq + PartialOrd,
{
    // OuterBitmapIterator
    // Pass F = () and parent_key = SlotKey<MarketPreimage<M, B, Q>>
    //
    // InnerBitmapIterator
    // Pass F = OuterBitmapIndex and parent_key = SlotKey<OuterBitmapPreimage<M, B, Q>>
    fn build_iterator<M, B, Q, In>(
        market_key: SlotKey<MarketPreimage<M, B, Q>>,
        range: RangeInclusive<(P0, P1)>,
    ) -> impl Iterator<Item = BitmapEntry<F, P0, P1, Self>>
    where
        M: MarketMarker,
        B: TokenMarker,
        Q: TokenMarker,
        In: LegMatcher,
    {
        let (start, end) = range.into_inner();

        // TODO define BitmapIndex trait that maps In: LegMarker and RangeInclusive<P0> to iterator
        let outer_range = start.0..=end.0;

        P0::build_iterator(outer_range).filter_map(move |p0| {
            // TODO build key
            // Preimage is derived from market_key, P0 and F
            //
            // 1. OuterBitmap: no F
            // 2. InnerBitmap: we use the hash of the OuterBitmapIndex
            //
            // Hack- we can't pass market_key. Instead we need a generic
            // that takes both MarketKey and OuterBitmapkey.
            // Define ForeignKey or ParentIndex trait
            //
            // Alternative- inner hash uses MarketKey, OuterBitmapIndex and InnerBitmapIndex.
            // But this is more expensive.
        })

        // Declare outer iterator on start.0 and end.0, i.e. on RangeInclusive<P0>
        // 1. OuterBitmap: This is RangeInclusive<OuterBitmap>
        // 2. InnerBitmap: This is RangeInclusive<(OuterBitmap, InnerBitmap)>.
        // We then match iterator position with start and end to update the inner iterator bounds.
        // let index_range = OuterBitmapIndexV2::convert_range(range);

        // In::outer_bitmap_index_iter(index_range)
    }
}
