use crate::{
    settlement::local_delta::MakerDelta,
    types::{Base, Pair, Quote, Tuple},
};

/// Maker deltas for base and quote sides
pub type MakerDeltaPair = Pair<MakerDelta<Base>, MakerDelta<Quote>>;

impl MakerDeltaPair {
    pub const fn new() -> Self {
        Tuple::new2(MakerDelta::<Base>::new(), MakerDelta::<Quote>::new())
    }
}
