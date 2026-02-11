use crate::{
    axis::leg::{Base, Pair, Quote},
    settlement::local_delta::TakerDelta,
};

/// The results of matching take orders
pub type TakerDeltaPair = Pair<TakerDelta<Base>, TakerDelta<Quote>>;

impl TakerDeltaPair {
    pub const fn zero() -> Self {
        Self::new(TakerDelta::<Base>::zero(), TakerDelta::<Quote>::zero())
    }
}
