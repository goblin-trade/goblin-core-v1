use crate::{
    markets::{Hardcoded, MarketWithKey},
    token::TokenMarker,
};

/// Map each PairShape to a hardcoded market list
pub trait HardcodedMarketList<B: TokenMarker + 'static, Q: TokenMarker + 'static> {
    const HARDCODED_MARKET_LIST: &'static [MarketWithKey<Hardcoded, B, Q>];
}
