use crate::{axis::leg::SamePair, quantities::Position};

/// The market state slot
/// We have 6 possible sub-types based on MarketVariant and PairShape
#[repr(C)]
pub struct MarketState {
    /// The last known price coordinates at the centre
    pub last_coordinates: SamePair<Position>,
    /// Padding to match 32 bits
    _padding: [u8; 16],
}
