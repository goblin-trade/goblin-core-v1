use crate::{
    settlement::local_delta::TakerDelta,
    types::{Base, Pair, Quote, Tuple},
};

/// The results of matching take orders
pub type TakerDeltaPair = Pair<TakerDelta<Base>, TakerDelta<Quote>>;

impl TakerDeltaPair {
    pub const fn zero() -> Self {
        Tuple::new2(TakerDelta::<Base>::zero(), TakerDelta::<Quote>::zero())
    }
}
