use goblin_macros::fixed_codec;

use crate::axis::leg::SamePair;

#[fixed_codec(bits = 8)]
pub struct MarketHeader {
    /// Whether to decode deposit amounts
    pub decode_deposit_amounts: bool,

    /// Whether to execute take orders for sides In=Base and In=Quote
    pub execute_takes: SamePair<bool>,

    /// Number of outer bitmaps to traverse
    pub outer_bitmap_count: u8,
}
