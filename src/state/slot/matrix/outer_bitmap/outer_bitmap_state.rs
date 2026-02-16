use crate::{
    axis::{market::market_marker::MarketMarker, token::token_marker::TokenMarker},
    matching::bitmap::OuterBitmapIndex,
    state::{outer_bitmap::OuterBitmap, MarketPreimage, Preimage, SlotKey},
};

pub const CLOSED_SENTINEL: OuterBitmap = OuterBitmap([0xFF; 32]);

pub enum OuterBitmapState {
    Closed,
    Active([u8; 32]),
}

impl From<OuterBitmap> for OuterBitmapState {
    fn from(value: OuterBitmap) -> Self {
        if value == CLOSED_SENTINEL {
            OuterBitmapState::Closed
        } else {
            OuterBitmapState::Active(value.0)
        }
    }
}
