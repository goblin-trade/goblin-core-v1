use crate::{
    axis::leg::{Base, Pair, Quote},
    settlement::{local_delta::MakerDeltaPair, MatchedLots},
    types::{StoreReader, Tuple},
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
