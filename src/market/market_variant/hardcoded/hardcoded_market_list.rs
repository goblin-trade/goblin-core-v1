use crate::{
    market::{Hardcoded, MarketAndKey},
    token::TokenMarker,
};

/// Map each TokenMarker combination to a hardcoded list
pub trait HardcodedMarketList<B: TokenMarker, Q: TokenMarker> {
    const HARDCODED_MARKET_LIST: &'static [MarketAndKey<Hardcoded, B, Q>];
}
