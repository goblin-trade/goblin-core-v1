pub mod hardcoded_market_list;

pub use hardcoded_market_list::*;

use crate::{
    axis::{
        market::{Hardcoded, MarketIndex, MarketLocator},
        token::TokenDataTriple,
    },
    axis_helpers::TokenPair,
    goblin_error::GoblinError,
    market::MarketReadables,
};

impl<TP: TokenPair + HardcodedMarketList> MarketLocator<TP> for Hardcoded {
    type Locator = MarketIndex<TP>;

    fn locate_market(
        locator: &Self::Locator,
        _token_data_triple: &TokenDataTriple,
    ) -> Result<MarketReadables<TP>, GoblinError> {
        Ok(TP::HARDCODED_MARKET_LIST[*locator])
    }
}
