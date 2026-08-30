use crate::{
    axis::market::{Dynamic, Hardcoded, MarketEnum},
    axis_helpers::AxisMarker,
};

///! We have 2 market variants
///!
///! * Hardcoded market- has hardcoded tokens
///! * Dynamic market- has dynamic tokens that can be either hardcoded or custom
pub trait MarketMarker:
    Sized + Clone + Copy + PartialEq + PartialOrd + AxisMarker<Enum = MarketEnum>
{
    /// Discriminator used to hash the market key
    ///
    /// TODO unused, remove
    const DISCRIMINATOR: u8;
}

impl MarketMarker for Hardcoded {
    const DISCRIMINATOR: u8 = 3;
}

impl MarketMarker for Dynamic {
    const DISCRIMINATOR: u8 = 4;
}
