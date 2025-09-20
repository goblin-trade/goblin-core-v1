use crate::{
    goblin_error::GoblinError,
    markets::{IndexedMarket, HARDCODED_MARKETS},
    require,
};

/// Index representing a market from the entered custom market list or hardcoded market list
pub struct MarketIndex(pub u8);

impl MarketIndex {
    /// Obtain market for the given MarketIndex
    ///
    /// We use the index number to lookup in custom and hardcoded market lists.
    /// Custom markets are tested for validity.
    ///
    /// # Arguments
    ///
    /// * `dangerous_custom_market_list` - Custom market list read from args
    pub fn to_indexed_market(
        &self,
        dangerous_custom_market_list: &[IndexedMarket],
    ) -> Result<IndexedMarket, GoblinError> {
        let index = self.0 as usize;

        if index < dangerous_custom_market_list.len() {
            let dangerous_custom_market = dangerous_custom_market_list[index];
            require!(
                dangerous_custom_market.is_valid(),
                GoblinError::InvalidMarket
            );

            Ok(dangerous_custom_market)
        } else if index > 127 && index < (127 + HARDCODED_MARKETS.len()) {
            Ok(HARDCODED_MARKETS[index - 127])
        } else {
            Err(GoblinError::NoMarketAtIndex)
        }
    }
}
