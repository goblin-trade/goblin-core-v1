use crate::{
    axis::{
        leg::leg_matcher::LegMatcher, market::market_marker::MarketMarker,
        token::token_marker::TokenMarker,
    },
    matching::{
        active_iterator::inner_bitmap::inner_bitmap_item::InnerBitmapItem,
        bitmap::{outer_bitmap_index::OuterBitmapIndex, outer_pos::OuterPos},
    },
    state::{
        bitmap::{
            inner_bitmap::preimage::InnerBitmapPreimage,
            outer_bitmap::{active_outer_bitmap::ActiveOuterBitmap, preimage::OuterBitmapPreimage},
            Bitmap,
        },
        Preimage, SlotKey,
    },
};

pub struct OuterBitmapItemV2<M, B, Q, In>
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
}
