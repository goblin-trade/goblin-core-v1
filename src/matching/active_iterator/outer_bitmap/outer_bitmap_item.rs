use crate::{
    axis::{
        leg::leg_matcher::LegMatcher, market::market_marker::MarketMarker,
        token::token_marker::TokenMarker,
    },
    matching::bitmap::outer_bitmap_index::OuterBitmapIndex,
    state::{
        outer_bitmap::{active_outer_bitmap::ActiveOuterBitmap, preimage::OuterBitmapPreimage},
        SlotKey,
    },
};

pub struct OuterBitmapItem<M, B, Q, In>
where
    M: MarketMarker,
    B: TokenMarker,
    Q: TokenMarker,
    In: LegMatcher,
{
    pub outer_bitmap_index: OuterBitmapIndex<In>,
    pub outer_bitmap_key: SlotKey<OuterBitmapPreimage<M, B, Q, In>>,
    pub active_outer_bitmap: ActiveOuterBitmap<M, B, Q>,
}
