use crate::axis::market::{market_marker::MarketMarker, Dynamic};

impl MarketMarker for Dynamic {
    const DISCRIMINATOR: u8 = 4;
}
