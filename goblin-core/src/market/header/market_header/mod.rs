use deku::DekuRead;
#[cfg(feature = "encode")]
use deku::DekuWrite;

use crate::axis::leg::SamePair;

#[derive(DekuRead)]
#[deku(bit_order = "lsb")]
#[cfg_attr(feature = "encode", derive(DekuWrite))]
pub struct MarketHeader {
    /// Whether to decode deposit amounts
    #[deku(bits = "1")]
    pub decode_deposit_amounts: bool,

    /// Whether to execute take orders for sides In=Base and In=Quote
    #[deku(bits = "2", ctx = "(1, 1)")]
    pub execute_takes: SamePair<bool>,

    /// Number of outer bitmaps to traverse
    #[deku(bits = "5")]
    pub outer_bitmap_count: u8,
}
