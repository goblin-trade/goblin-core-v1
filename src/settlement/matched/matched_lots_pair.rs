use crate::{
    axis::leg::{Base, Pair, Quote},
    settlement::MatchedLots,
};

pub type MatchedLotsPair = Pair<MatchedLots<Base>, MatchedLots<Quote>>;
