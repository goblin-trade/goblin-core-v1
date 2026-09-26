mod impl_goblin_read;

#[cfg(feature = "encode")]
mod impl_goblin_write;

use crate::{
    axis::leg::SamePair, axis_helpers::MarketSpec, settlement::local_delta::LocalDeposits,
};

/// Per-market header, packed into a single byte LSB-first.
///
/// The byte is decoded as a lane: the trailing `outer_bitmap_count` takes
/// whatever is left after the flag fields (5 bits).
///
/// The leading byte is followed by the market locator, and then by the local
/// deposits when `decode_deposit_amounts` is set.
pub struct MarketHeader<MS: MarketSpec> {
    /// Whether to decode deposit amounts
    pub decode_deposit_amounts: bool,

    /// Whether to execute take orders for sides In=Base and In=Quote
    pub execute_takes: SamePair<bool>,

    /// Number of outer bitmaps to traverse
    pub outer_bitmap_count: u8,

    /// Intermediate representation used to locate this market.
    pub locator: MS::Locator,

    /// Pending local deposits, decoded only when `decode_deposit_amounts` is
    /// set, otherwise defaulted.
    pub local_deposits: LocalDeposits<MS::Pair>,
}
