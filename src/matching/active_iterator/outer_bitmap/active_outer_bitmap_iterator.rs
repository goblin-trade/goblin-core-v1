use crate::{
    axis::{
        leg::leg_matcher::LegMatcher, market::market_marker::MarketMarker,
        token::token_marker::TokenMarker,
    },
    matching::{
        active_iterator::outer_bitmap::market_item::MarketItem,
        bitmap::outer_bitmap_index::OuterBitmapIndex,
    },
    state::{MarketPreimage, SlotKey},
};
use core::ops::RangeInclusive;

/// Return active outer bitmaps with their index
pub struct ActiveOuterBitmapIterator<'a, M, B, Q, In>
where
    M: MarketMarker,
    B: TokenMarker,
    Q: TokenMarker,
    In: LegMatcher,
{
    /// Market key
    pub item: MarketItem<'a, M, B, Q, In>,

    /// Linear iterator
    pub linear_iterator: In::OuterBitmapIndexIter,

    /// Stop when limit reached
    pub limit: OuterBitmapIndex<In>,
}

impl<'a, M, B, Q, In> ActiveOuterBitmapIterator<'a, M, B, Q, In>
where
    M: MarketMarker,
    B: TokenMarker,
    Q: TokenMarker,
    In: LegMatcher,
{
    pub fn new(
        market_key: &'a SlotKey<MarketPreimage<M, B, Q>>,
        range: RangeInclusive<OuterBitmapIndex<In>>,
    ) -> Self {
        Self {
            item: MarketItem::new(market_key),
            limit: *range.end(),
            linear_iterator: In::outer_bitmap_index_iter(range),
        }
    }
}
