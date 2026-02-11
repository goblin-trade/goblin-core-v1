use crate::{
    axis::{
        leg::{Base, Pair, Quote},
        market::LotSizePair,
    },
    settlement::{global_delta::GlobalMakerUpdate, local_delta::MakerDeltaPair},
};

pub type GlobalMakerUpdatePair = Pair<GlobalMakerUpdate<Base>, GlobalMakerUpdate<Quote>>;

impl GlobalMakerUpdatePair {
    pub fn new_pair(maker_delta_pair: &MakerDeltaPair, lot_size_pair: &LotSizePair) -> Self {
        Self::new(
            GlobalMakerUpdate::<Base>::new(maker_delta_pair, lot_size_pair),
            GlobalMakerUpdate::<Quote>::new(maker_delta_pair, lot_size_pair),
        )
    }
}
