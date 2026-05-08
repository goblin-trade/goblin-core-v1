use crate::{
    axis::{market::market_marker::MarketMarker, token::token_marker::TokenMarker},
    quantities::{bits_layout::BitsLayout, Position},
    state::{bitmap::Bitmap, MarketPreimage, Preimage, SlotKey},
};

#[repr(C)]
#[derive(Clone, Copy)]
pub struct BitmapPreimage<M, B, Q, const BITS: u16>
where
    M: MarketMarker,
    B: TokenMarker,
    Q: TokenMarker,
{
    pub market_key: SlotKey<MarketPreimage<M, B, Q>>,
    pub position: Position,
}

impl<M, B, Q, const BITS: u16> Preimage for BitmapPreimage<M, B, Q, BITS>
where
    M: MarketMarker,
    B: TokenMarker,
    Q: TokenMarker,
{
    const SLOT_DISCRIMINATOR: u8 = 5 + BitsLayout::<BITS>::OFFSET as u8;
    type SlotState = Bitmap<BITS>;
}
