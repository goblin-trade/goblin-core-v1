use crate::{
    axis::leg::{Base, Pair, Quote},
    settlement::MatchedLots,
};

/// The sender delta of local namespace
///
/// Values are denominated in MatchingLots. Convert it to TakerTokenUpdate
/// so it can be added to the global delta
pub struct LocalSenderDelta {
    /// The results of matching take orders
    pub taker_delta_pair: Pair<MatchedLots<Base>, MatchedLots<Quote>>,
}

impl LocalSenderDelta {
    pub const fn zero() -> Self {
        Self {
            taker_delta_pair: Pair::new(MatchedLots::<Base>::zero(), MatchedLots::<Quote>::zero()),
        }
    }
}
