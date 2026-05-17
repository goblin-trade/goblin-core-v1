use crate::{axis::leg::SamePair, impl_checked_slot_state, quantities::Pos2};

/// The market state slot
/// We have 6 possible sub-types based on MarketVariant and PairShape
#[repr(C)]
pub struct MarketState {
    /// The last known price positions at the centre
    pub last_positions: SamePair<Pos2>,
    /// Padding to match 32 bits
    _padding: [u8; 16],
}
impl_checked_slot_state!(MarketState);
