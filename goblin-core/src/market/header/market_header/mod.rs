mod impl_goblin_read;

#[cfg(feature = "encode")]
mod impl_goblin_write;

use crate::{axis::leg::SamePair, axis_helpers::TokenPair, settlement::local_delta::LocalDeposits};

/// Per-market header, packed into a single byte LSB-first.
///
/// The byte is decoded as a lane: the trailing `outer_bitmap_count` takes
/// whatever is left after the flag fields (5 bits).
///
/// The leading byte is followed by the local deposits when
/// `decode_deposit_amounts` is set.
pub struct MarketHeader<TP: TokenPair> {
    /// Whether to decode deposit amounts
    pub decode_deposit_amounts: bool,

    /// Whether to execute take orders for sides In=Base and In=Quote
    pub execute_takes: SamePair<bool>,

    /// Number of outer bitmaps to traverse
    pub outer_bitmap_count: u8,

    /// Pending local deposits, decoded only when `decode_deposit_amounts` is
    /// set, otherwise defaulted.
    pub local_deposits: LocalDeposits<TP>,
}
