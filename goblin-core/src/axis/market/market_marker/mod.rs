use crate::{
    axis::market::{Dynamic, Hardcoded, MarketEnum},
    axis_helpers::AxisMarker,
    input_processor::{MarketCounts, MarketCountsInner},
    types::StoreReader,
};

pub trait MarketMarker:
    Clone
    + Copy
    + PartialEq
    + PartialOrd
    + AxisMarker<Enum = MarketEnum>
    + StoreReader<MarketCounts, Result = MarketCountsInner>
{
}

impl MarketMarker for Hardcoded {}

impl MarketMarker for Dynamic {}
