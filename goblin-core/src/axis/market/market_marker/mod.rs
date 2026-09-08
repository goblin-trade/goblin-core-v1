use crate::{
    axis::market::{Dynamic, Hardcoded, MarketEnum},
    axis_helpers::AxisMarker,
};

pub trait MarketMarker:
    Clone + Copy + PartialEq + PartialOrd + AxisMarker<Enum = MarketEnum>
{
}

impl MarketMarker for Hardcoded {}

impl MarketMarker for Dynamic {}
