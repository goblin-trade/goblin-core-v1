use crate::axis::market::{Dynamic, Hardcoded};

///! We have 2 market variants
///!
///! * Hardcoded market- has hardcoded tokens
///! * Dynamic market- has dynamic tokens that can be either hardcoded or custom
pub trait MarketMarker: Sized + Clone + Copy {
    /// Discriminator used to hash the market key
    const DISCRIMINATOR: u8;
}

impl MarketMarker for Hardcoded {
    const DISCRIMINATOR: u8 = 3;
}

impl MarketMarker for Dynamic {
    const DISCRIMINATOR: u8 = 4;
}
