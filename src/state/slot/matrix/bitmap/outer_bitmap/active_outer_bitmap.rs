use crate::{
    axis::{
        leg::{leg_iterator::LegIterator, leg_matcher::LegMatcher, Base, Quote, SamePair},
        market::market_marker::MarketMarker,
        token::token_marker::TokenMarker,
    },
    state::{
        bitmap::{
            outer_bitmap::{outer_bitmap_state::OuterBitmapState, preimage::OuterBitmapPreimage},
            Bitmap,
        },
        SlotKey,
    },
    types::StoreReader,
};

#[repr(C)]
#[derive(PartialEq, Default)]
pub struct ActiveOuterBitmap {
    pub inner: [u8; 32],
}

impl ActiveOuterBitmap {
    pub const fn new(inner: [u8; 32]) -> Self {
        Self { inner }
    }

    // pub fn new_cleaned_v2<M, B, Q>(
    //     key: &SlotKey<OuterBitmapPreimage<M, B, Q>>,
    //     outer_bitmap_index: OuterBitmapIndex,
    //     outer_bitmap_index_pair: &SamePair<OuterBitmapIndex>,
    // ) -> Self
    // where
    //     M: MarketMarker,
    //     B: TokenMarker,
    //     Q: TokenMarker,
    // {
    //     if outer_bitmap_index.holds_garbage(outer_bitmap_index_pair) {
    //         Self::default()
    //     } else {
    //         if let OuterBitmapState::Active(active_outer_bitmap) =
    //             OuterBitmapState::from(key.load())
    //         {
    //             active_outer_bitmap
    //         } else {
    //             Self::default()
    //         }
    //     }
    // }
}
