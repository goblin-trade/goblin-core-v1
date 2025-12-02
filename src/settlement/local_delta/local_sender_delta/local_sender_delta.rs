use crate::{
    settlement::local_delta::TakerDelta,
    types::{Base, Pair, Quote},
};

/// The sender delta of local namespace
///
/// Values are denominated in MatchingLots. Convert it to TakerTokenUpdate
/// so it can be added to the global delta
pub struct LocalSenderDelta {
    /// The results of matching take orders
    pub taker_delta_pair: Pair<TakerDelta<Base>, TakerDelta<Quote>>,
}

impl LocalSenderDelta {
    pub const fn new() -> Self {
        Self {
            taker_delta_pair: Pair {
                base: TakerDelta::<Base>::new(),
                quote: TakerDelta::<Quote>::new(),
            },
        }
    }
}
