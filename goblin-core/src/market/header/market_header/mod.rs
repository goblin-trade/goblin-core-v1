mod impl_deku_reader;

#[cfg(feature = "encode")]
mod impl_deku_writer;

use crate::axis::leg::SamePair;

/// Per-market header, packed into a single byte LSB-first.
///
/// deku's `bits` feature is off, so the byte is decoded as a lane: the trailing
/// `outer_bitmap_count` takes whatever is left after the flag fields (5 bits).
pub struct MarketHeader {
    /// Whether to decode deposit amounts
    pub decode_deposit_amounts: bool,

    /// Whether to execute take orders for sides In=Base and In=Quote
    pub execute_takes: SamePair<bool>,

    /// Number of outer bitmaps to traverse
    pub outer_bitmap_count: u8,
}
