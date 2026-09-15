pub mod market_preimage;

pub use market_preimage::*;

use crate::{axis::leg::SamePair, impl_checked_slot_state, quantities::Position};

/// The market state slot
#[repr(C)]
pub struct MarketState {
    /// The last known price positions at the centre
    pub last_positions: SamePair<Position>,
    /// Padding to match 32 bits
    _padding: [u8; 16],
}
impl_checked_slot_state!(MarketState);
