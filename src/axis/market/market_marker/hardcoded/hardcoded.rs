use crate::axis::market::{market_marker::MarketMarker, Hardcoded};

impl MarketMarker for Hardcoded {
    const DISCRIMINATOR: u8 = 3;
}
