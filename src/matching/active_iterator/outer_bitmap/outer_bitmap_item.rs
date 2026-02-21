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
    /// Index of the active outer bitmap
    pub outer_bitmap_index: OuterBitmapIndex<In>,

    /// The slot key
    pub outer_bitmap_key: SlotKey<OuterBitmapPreimage<M, B, Q, In>>,

    /// The active outer bitmap read from slot
    pub active_outer_bitmap: ActiveOuterBitmap<M, B, Q>,

    /// Whether `limit` is reached and that this is the last value
    pub limit_reached: bool,
}
