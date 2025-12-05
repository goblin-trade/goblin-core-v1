use crate::{
    settlement::{
        local_delta::{MakerDeltaPair, TakerDeltaPair},
        MatchedLots,
    },
    types::{Base, Pair, Quote},
};

pub type MatchedLotsPair = Pair<MatchedLots<Base>, MatchedLots<Quote>>;

impl From<&MakerDeltaPair> for MatchedLotsPair {
    fn from(value: &MakerDeltaPair) -> Self {
        Pair {
            base: value.base.matched_lots,
            quote: value.quote.matched_lots,
        }
    }
}

impl From<&TakerDeltaPair> for MatchedLotsPair {
    fn from(value: &TakerDeltaPair) -> Self {
        Pair {
            base: value.base.matched_lots,
            quote: value.quote.matched_lots,
        }
    }
}
