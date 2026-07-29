use crate::{
    axis::market::market_spec::MarketSpec,
    quantities::{bits_layout::BitsLayout, SafePosition, OUTER_POS, POS_0},
    state::{bitmap::Bitmap, MarketPreimage, Preimage, SlotKey, SlotState},
};

#[repr(C)]
#[derive(Clone, Copy)]
pub struct BitmapPreimage<MS: MarketSpec, const BITS: u16, const INNER_BITS: u16> {
    pub market_key: SlotKey<MarketPreimage<MS>>,
    pub safe_position: SafePosition<BITS>,
}

impl<MS: MarketSpec, const BITS: u16, const INNER_BITS: u16> Preimage
    for BitmapPreimage<MS, BITS, INNER_BITS>
{
    const SLOT_DISCRIMINATOR: u8 = 5 + BitsLayout::<BITS>::OFFSET as u8;
    type SlotState = Bitmap<BITS, INNER_BITS>;
}

unsafe impl<const BITS: u16, const INNER_BITS: u16> SlotState for Bitmap<BITS, INNER_BITS> {}
const _: () = <Bitmap<POS_0, OUTER_POS> as SlotState>::_ASSERT;
