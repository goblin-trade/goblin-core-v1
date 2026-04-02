use crate::{
    axis::{
        leg::{Base, Quote, SamePair},
        market::market_marker::MarketMarker,
        token::token_marker::TokenMarker,
    },
    matching::bitmap::{
        inner_pos::InnerPos, outer_bitmap_index::OuterBitmapIndex, outer_pos::OuterPos,
    },
    state::{
        bitmap::{
            inner_bitmap::preimage::InnerBitmapPreimage,
            outer_bitmap::active_outer_bitmap::ActiveOuterBitmap, Bitmap,
        },
        SlotKey,
    },
    types::StoreReader,
};

#[repr(C)]
#[derive(PartialEq, Default)]
pub struct InnerBitmap {
    pub inner: [u8; 32],
}

impl InnerBitmap {
    pub const fn new(inner: [u8; 32]) -> Self {
        Self { inner }
    }

    pub fn new_cleaned<M, B, Q>(
        key: &SlotKey<InnerBitmapPreimage<M, B, Q>>,
        active_outer_bitmap: &ActiveOuterBitmap,
        outer_bitmap_index: OuterBitmapIndex,
        outer_pos: OuterPos,
        outer_bitmap_index_pair: &SamePair<OuterBitmapIndex>,
        outer_pos_pair: &SamePair<OuterPos>,
        inner_pos_pair: &SamePair<InnerPos>,
    ) -> Self
    where
        M: MarketMarker,
        B: TokenMarker,
        Q: TokenMarker,
    {
        if !active_outer_bitmap.pos_active(outer_pos) {
            InnerBitmap::default()
        } else {
            let inner_bitmap = key.load();

            // remove garbage bits if on limit
            // TODO BitmapReader trait that maps
            // 1. OuterBitmap -> (outer_bitmap_index)
            // 2. InnerBitmap -> (outer_bitmap_index, outer_pos)
            // Use it to simplify equality checks
            let on_last_base = outer_bitmap_index == Base::get(outer_bitmap_index_pair)
                && outer_pos == Base::get(outer_pos_pair);
            let on_last_quote = outer_bitmap_index == Quote::get(outer_bitmap_index_pair)
                && outer_pos == Quote::get(outer_pos_pair);

            // We only clean rows not columns
            // Each byte corresponds to a row

            inner_bitmap
        }
    }
}
