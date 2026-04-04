use crate::{
    axis::{market::market_marker::MarketMarker, token::token_marker::TokenMarker},
    quantities::OuterPosV2,
    state::{
        bitmap::{inner_bitmap::InnerBitmap, outer_bitmap::preimage::OuterBitmapPreimage},
        Preimage, SlotKey,
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
    pub outer_bitmap_key: SlotKey<OuterBitmapPreimage<M, B, Q>>,
    pub outer_pos: OuterPosV2,
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
