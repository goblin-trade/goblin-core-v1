use crate::{
    markets::LotSizePair,
    settlement::{global_delta::GlobalUpdate, local_delta::TakerDelta},
    types::{Base, Pair, Quote},
};

pub type GlobalUpdatePair = Pair<GlobalUpdate<Base>, GlobalUpdate<Quote>>;

impl GlobalUpdatePair {
    pub fn new(
        taker_delta_pair: &Pair<TakerDelta<Base>, TakerDelta<Quote>>,
        lot_size_pair: &LotSizePair,
    ) -> Self {
        Pair {
            base: GlobalUpdate::<Base>::new(taker_delta_pair, lot_size_pair),
            quote: GlobalUpdate::<Quote>::new(taker_delta_pair, lot_size_pair),
        }
    }
}
