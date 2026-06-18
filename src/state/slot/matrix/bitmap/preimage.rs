use crate::{
    axis::{market::market_marker::MarketMarker, token::token_reader::TokenReader},
    quantities::{bits_layout::BitsLayout, SafePosition, OUTER_POS, POS_0},
    state::{bitmap::Bitmap, MarketPreimage, Preimage, SlotKey, SlotState},
};

#[repr(C)]
#[derive(Clone, Copy)]
pub struct BitmapPreimage<M, B, Q, const BITS: u16, const INNER_BITS: u16>
where
    M: MarketMarker,
    B: TokenReader,
    Q: TokenReader,
{
    pub market_key: SlotKey<MarketPreimage<M, B, Q>>,
    pub safe_position: SafePosition<BITS>,
}

impl<M, B, Q, const BITS: u16, const INNER_BITS: u16> Preimage
    for BitmapPreimage<M, B, Q, BITS, INNER_BITS>
where
    M: MarketMarker,
    B: TokenReader,
    Q: TokenReader,
{
    const SLOT_DISCRIMINATOR: u8 = 5 + BitsLayout::<BITS>::OFFSET as u8;
    type SlotState = Bitmap<BITS, INNER_BITS>;
}

unsafe impl<const BITS: u16, const INNER_BITS: u16> SlotState for Bitmap<BITS, INNER_BITS> {}
const _: () = <Bitmap<POS_0, OUTER_POS> as SlotState>::_ASSERT;
