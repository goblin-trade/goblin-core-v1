use crate::{
    axis::market::{Dynamic, Hardcoded, MarketEnum},
    axis_helpers::AxisMarker,
    input_processor::{MarketCountsInner, MarketCountsV2},
    types::StoreReader,
};

pub trait MarketMarker:
    Clone
    + Copy
    + PartialEq
    + PartialOrd
    + AxisMarker<Enum = MarketEnum>
    + StoreReader<MarketCountsV2, Result = MarketCountsInner>
{
}

impl MarketMarker for Hardcoded {}

impl MarketMarker for Dynamic {}
