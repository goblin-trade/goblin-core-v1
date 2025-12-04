use crate::{
    markets::LotSizePair,
    settlement::{global_delta::GlobalSenderUpdate, local_delta::TakerDelta},
    types::{Base, Pair, Quote},
};

pub type GlobalSenderUpdatePair = Pair<GlobalSenderUpdate<Base>, GlobalSenderUpdate<Quote>>;

impl GlobalSenderUpdatePair {
    pub fn new(
        taker_delta_pair: &Pair<TakerDelta<Base>, TakerDelta<Quote>>,
        lot_size_pair: &LotSizePair,
    ) -> Self {
        Pair {
            base: GlobalSenderUpdate::<Base>::new(taker_delta_pair, lot_size_pair),
            quote: GlobalSenderUpdate::<Quote>::new(taker_delta_pair, lot_size_pair),
        }
    }
}
