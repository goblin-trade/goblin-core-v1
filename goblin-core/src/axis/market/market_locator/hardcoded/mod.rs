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
    type Readables = &'static MarketReadables<TP>;

    fn locate_market(
        locator: &Self::Locator,
        _token_data_triple: &TokenDataTriple,
    ) -> Result<Self::Readables, GoblinError> {
        // Borrow straight out of the static list: never copied.
        Ok(&TP::HARDCODED_MARKET_LIST[*locator])
    }
}
