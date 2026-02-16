use crate::{
    axis::{market::market_marker::MarketMarker, token::token_marker::TokenMarker},
    matching::bitmap::OuterBitmapIndex,
    state::{outer_bitmap::OuterBitmap, MarketPreimage, Preimage, SlotKey},
};

#[repr(C)]
#[derive(Clone, Copy)]
pub struct OuterBitmapPreimage<M, B, Q>
where
    M: MarketMarker,
    B: TokenMarker,
    Q: TokenMarker,
{
    pub market_key: SlotKey<MarketPreimage<M, B, Q>>,
    pub outer_bitmap_index: OuterBitmapIndex,
}

impl<M, B, Q> Preimage for OuterBitmapPreimage<M, B, Q>
where
    M: MarketMarker,
    B: TokenMarker,
    Q: TokenMarker,
{
    const SLOT_DISCRIMINATOR: u8 = 5;
    type SlotState = OuterBitmap;
}
