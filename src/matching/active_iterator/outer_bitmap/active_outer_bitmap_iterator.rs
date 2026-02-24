use crate::{
    axis::{
        leg::leg_matcher::LegMatcher, market::market_marker::MarketMarker,
        token::token_marker::TokenMarker,
    },
    matching::bitmap::{outer_bitmap_index::OuterBitmapIndex, range::Range},
    state::{MarketPreimage, SlotKey},
};

/// Return active outer bitmaps with their index
pub struct ActiveOuterBitmapIterator<'a, M, B, Q, In>
where
    M: MarketMarker,
    B: TokenMarker,
    Q: TokenMarker,
    In: LegMatcher,
{
    /// Market key
    pub market_key: &'a SlotKey<MarketPreimage<M, B, Q>>,

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
        range: Range<OuterBitmapIndex<In>>,
    ) -> Self {
        Self {
            market_key,
            linear_iterator: In::outer_bitmap_index_iter(range.start),
            limit: range.limit,
        }
    }
}
