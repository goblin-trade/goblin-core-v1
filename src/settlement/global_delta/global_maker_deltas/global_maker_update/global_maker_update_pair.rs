use crate::{
    markets::LotSizePair,
    settlement::{global_delta::GlobalMakerUpdate, local_delta::MakerDeltaPair},
    types::{Base, Pair, Quote, Tuple},
};

pub type GlobalMakerUpdatePair = Pair<GlobalMakerUpdate<Base>, GlobalMakerUpdate<Quote>>;

impl GlobalMakerUpdatePair {
    pub fn new(maker_delta_pair: &MakerDeltaPair, lot_size_pair: &LotSizePair) -> Self {
        Tuple::new2(
            GlobalMakerUpdate::<Base>::new(maker_delta_pair, lot_size_pair),
            GlobalMakerUpdate::<Quote>::new(maker_delta_pair, lot_size_pair),
        )
    }
}
