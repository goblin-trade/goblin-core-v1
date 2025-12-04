use crate::quantities::QuantityOps;
use crate::types::LegMarker;

/// Maker delta for taker side In
///
/// # Convention
///
/// * `In` represents the taker side. Maker side is opposite of `In`.
/// * If In = Base, then the maker fills the side Quote.
///
#[derive(Default, Clone, Copy, PartialEq)]
pub struct MakerDelta<In: LegMarker> {
    /// Input token gained by maker
    pub taker_in: In::MatchingLots,

    /// Locked output token released by maker
    pub taker_out: <In::Opposite as LegMarker>::MatchingLots,
}

impl<In: LegMarker> MakerDelta<In> {
    pub const fn new() -> Self {
        Self {
            taker_in: In::MatchingLots::ZERO,
            taker_out: <In::Opposite as LegMarker>::MatchingLots::ZERO,
        }
    }

    pub fn not_empty(&self) -> bool {
        *self != Self::new()
    }
}
