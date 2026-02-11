use crate::{
    axis::{
        leg::{Base, Pair, Quote},
        market::LotSizePair,
    },
    settlement::{global_delta::GlobalSenderUpdate, local_delta::TakerDelta},
};

pub type GlobalSenderUpdatePair = Pair<GlobalSenderUpdate<Base>, GlobalSenderUpdate<Quote>>;

impl GlobalSenderUpdatePair {
    pub fn new_pair(
        taker_delta_pair: &Pair<TakerDelta<Base>, TakerDelta<Quote>>,
        lot_size_pair: &LotSizePair,
    ) -> Self {
        Self::new(
            GlobalSenderUpdate::<Base>::new(taker_delta_pair, lot_size_pair),
            GlobalSenderUpdate::<Quote>::new(taker_delta_pair, lot_size_pair),
        )
    }
}
