use crate::{
    settlement::local_delta::TakerDelta,
    types::{Base, Pair, Quote},
};

/// The results of matching take orders
pub type TakerDeltaPair = Pair<TakerDelta<Base>, TakerDelta<Quote>>;

impl TakerDeltaPair {
    pub const fn new() -> Self {
        Self {
            base: TakerDelta::<Base>::new(),
            quote: TakerDelta::<Quote>::new(),
        }
    }
}
