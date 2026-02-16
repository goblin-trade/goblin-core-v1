use crate::{
    axis::{market::market_marker::MarketMarker, token::token_marker::TokenMarker},
    matching::bitmap::OuterPos,
    state::{
        inner_bitmap::InnerBitmap, outer_bitmap::preimage::OuterBitmapPreimage, Preimage, SlotKey,
    },
};

#[repr(C)]
#[derive(Clone, Copy)]
pub struct InnerBitmapPreimage<M, B, Q>
where
    M: MarketMarker,
    B: TokenMarker,
    Q: TokenMarker,
{
    outer_bitmap_key: SlotKey<OuterBitmapPreimage<M, B, Q>>,
    outer_pos: OuterPos,
}

impl<M, B, Q> Preimage for InnerBitmapPreimage<M, B, Q>
where
    M: MarketMarker,
    B: TokenMarker,
    Q: TokenMarker,
{
    const SLOT_DISCRIMINATOR: u8 = 6;
    type SlotState = InnerBitmap;
}
