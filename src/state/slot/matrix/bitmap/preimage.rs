use crate::{
    axis::{market::market_marker::MarketMarker, token::token_marker::TokenMarker},
    quantities::{bits_layout::BitsLayout, SafePosition, OUTER_POS},
    state::{bitmap::Bitmap, MarketPreimage, Preimage, SlotKey, SlotState},
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
    pub safe_position: SafePosition<BITS>,
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

unsafe impl<const BITS: u16> SlotState for Bitmap<BITS> {}
const _: () = <Bitmap<OUTER_POS> as SlotState>::_ASSERT;
