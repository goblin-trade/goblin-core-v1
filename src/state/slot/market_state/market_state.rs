use crate::{axis::leg::Pair, matching::bitmap::StoredCoordinates};

/// The market state slot
/// We have 6 possible sub-types based on MarketVariant and PairShape
#[repr(C)]
pub struct MarketState {
    /// The last known price coordinates at the centre
    /// The best price is equal to or worse than the last price.
    pub last_coordinates: Pair<StoredCoordinates, StoredCoordinates>,
    /// Padding to match 32 bits
    _padding: [u8; 14],
}
