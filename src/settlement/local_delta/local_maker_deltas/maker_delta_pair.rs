use crate::{
    settlement::local_delta::MakerDelta,
    types::{Base, Pair, Quote},
};

/// Maker deltas for base and quote sides
pub type MakerDeltaPair = Pair<MakerDelta<Base>, MakerDelta<Quote>>;

impl MakerDeltaPair {
    pub const fn new() -> Self {
        Self {
            base: MakerDelta::<Base>::new(),
            quote: MakerDelta::<Quote>::new(),
        }
    }
}
