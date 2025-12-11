use crate::{
    settlement::{
        local_delta::{MakerDeltaPair, TakerDeltaPair},
        MatchedLots,
    },
    types::{Base, Pair, Quote, Tuple, TupleReader},
};

pub type MatchedLotsPair = Pair<MatchedLots<Base>, MatchedLots<Quote>>;

impl From<&MakerDeltaPair> for MatchedLotsPair {
    fn from(value: &MakerDeltaPair) -> Self {
        Tuple::new(
            Base::get(value).matched_lots,
            Quote::get(value).matched_lots,
        )
    }
}

impl From<&TakerDeltaPair> for MatchedLotsPair {
    fn from(value: &TakerDeltaPair) -> Self {
        Tuple::new(
            Base::get(value).matched_lots,
            Quote::get(value).matched_lots,
        )
    }
}
