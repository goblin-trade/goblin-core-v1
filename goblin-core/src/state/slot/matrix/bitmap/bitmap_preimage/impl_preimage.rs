use crate::{
    axis_helpers::TokenPair,
    quantities::BitsLayout,
    state::{Bitmap, BitmapPreimage, Preimage},
};

impl<TP: TokenPair, const BITS: u16, const INNER_BITS: u16> Preimage
    for BitmapPreimage<TP, BITS, INNER_BITS>
{
    const SLOT_DISCRIMINATOR: u8 = 5 + BitsLayout::<BITS>::OFFSET as u8;
    type SlotState = Bitmap<BITS, INNER_BITS>;
}
