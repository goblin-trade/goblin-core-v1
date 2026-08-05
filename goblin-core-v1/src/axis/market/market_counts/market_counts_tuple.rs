use crate::{
    axis::market::{
        market_counts::{dynamic::DynamicCounts, hardcoded::HardcodedCounts},
        Market,
    },
    types::Tuple,
};

pub type MarketCountsTuple = Tuple<HardcodedCounts, DynamicCounts, Market>;
