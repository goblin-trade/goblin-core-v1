use crate::{
    axis::{
        leg::leg_matcher::LegMatcher, market::market_marker::MarketMarker,
        token::token_marker::TokenMarker,
    },
    matching::bitmap::{outer_bitmap_index::OuterBitmapIndex, outer_pos::OuterPos},
    state::{
        inner_bitmap::{preimage::InnerBitmapPreimage, InnerBitmap},
        SlotKey,
    },
};

pub struct InnerBitmapItem<M, B, Q, In>
where
    M: MarketMarker,
    B: TokenMarker,
    Q: TokenMarker,
    In: LegMatcher,
{
    pub outer_bitmap_index: OuterBitmapIndex<In>,
    pub outer_pos: OuterPos<In>,
    pub inner_bitmap_key: SlotKey<InnerBitmapPreimage<M, B, Q, In>>,
    pub inner_bitmap: InnerBitmap<M, B, Q>,
    pub limit_reached: bool,
}
