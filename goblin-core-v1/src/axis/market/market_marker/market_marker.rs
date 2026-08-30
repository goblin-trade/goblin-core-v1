///! We have 2 market variants
///!
///! * Hardcoded market- has hardcoded tokens
///! * Dynamic market- has dynamic tokens that can be either hardcoded or custom
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
