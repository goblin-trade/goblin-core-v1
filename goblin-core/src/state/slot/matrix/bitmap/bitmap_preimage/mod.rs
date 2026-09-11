mod impl_preimage;

use crate::{
    axis_helpers::TokenPair,
    quantities::SafePosition,
    state::{MarketPreimage, SlotKey},
};

#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct BitmapPreimage<TP: TokenPair, const BITS: u16, const INNER_BITS: u16> {
    pub market_key: SlotKey<MarketPreimage<TP>>,
    pub safe_position: SafePosition<BITS>,
}
