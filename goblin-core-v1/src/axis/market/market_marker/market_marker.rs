use crate::{
    axis::{
        market::{
            market_counts::{
                dynamic::DynamicCounts, hardcoded::HardcodedCounts, MarketCountsTuple,
            },
            Dynamic, Hardcoded, MarketEnum,
        },
        AxisMarker,
    },
    types::StoreReader,
};

///! We have 2 market variants
///!
///! * Hardcoded market- has hardcoded tokens
///! * Dynamic market- has dynamic tokens that can be either hardcoded or custom
pub trait MarketMarker:
    Sized
    + Clone
    + Copy
    + PartialEq
    + PartialOrd
    + AxisMarker<Enum = MarketEnum>
    + StoreReader<MarketCountsTuple, Result = Self::MarketCounts>
{
    /// Discriminator used to hash the market key
    const DISCRIMINATOR: u8;

    type MarketCounts;
}

impl MarketMarker for Hardcoded {
    const DISCRIMINATOR: u8 = 3;
    type MarketCounts = HardcodedCounts;
}

impl MarketMarker for Dynamic {
    const DISCRIMINATOR: u8 = 4;
    type MarketCounts = DynamicCounts;
}
