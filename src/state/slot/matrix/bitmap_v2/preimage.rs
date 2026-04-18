use crate::{
    axis::{market::market_marker::MarketMarker, token::token_marker::TokenMarker},
    quantities::{DerivedPosition, Position},
    state::{bitmap_v2::BitmapV2, MarketPreimage, Preimage, SlotKey},
};

#[repr(C)]
#[derive(Clone, Copy)]
pub struct BitmapPreimageV2<M, B, Q, const BITS: u16>
where
    M: MarketMarker,
    B: TokenMarker,
    Q: TokenMarker,
{
    pub market_key: SlotKey<MarketPreimage<M, B, Q>>,
    pub position: Position,
}

impl<M, B, Q, const BITS: u16> Preimage for BitmapPreimageV2<M, B, Q, BITS>
where
    M: MarketMarker,
    B: TokenMarker,
    Q: TokenMarker,
{
    // TODO update RestingOrderBitmap with discriminator 6
    const SLOT_DISCRIMINATOR: u8 = 5 + DerivedPosition::<u8, BITS>::BIT_OFFSET as u8;
    type SlotState = BitmapV2<BITS>;
}
