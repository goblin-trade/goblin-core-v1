use crate::{
    axis::leg::{Base, Pair, Quote},
    settlement::local_delta::MakerDelta,
};

/// Maker deltas for base and quote sides
pub type MakerDeltaPair = Pair<MakerDelta<Base>, MakerDelta<Quote>>;

impl MakerDeltaPair {
    pub const fn zero() -> Self {
        Self::new(MakerDelta::<Base>::zero(), MakerDelta::<Quote>::zero())
    }
}
