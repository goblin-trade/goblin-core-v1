use crate::{
    goblin_error::GoblinError,
    market::{DangerousMarketIndex, Hardcoded, MarketAndKey},
    token::TokenMarker,
};

/// Map each PairShape to a hardcoded market list
pub trait HardcodedMarketList<B: TokenMarker + 'static, Q: TokenMarker + 'static> {
    const HARDCODED_MARKET_LIST: &'static [MarketAndKey<Hardcoded, B, Q>];

    /// Get the hardcoded market and key corresponding to the market index
    ///
    /// Returns Error if index is out of bounds.
    fn get_market(
        index: DangerousMarketIndex<B, Q>,
    ) -> Result<&'static MarketAndKey<Hardcoded, B, Q>, GoblinError> {
        Self::HARDCODED_MARKET_LIST
            .get(index.inner)
            .ok_or(GoblinError::InvalidHardcodedMarket)
    }
}
