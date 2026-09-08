pub mod hardcoded_market_list;

pub use hardcoded_market_list::*;

use crate::{
    axis::{
        market::{Hardcoded, MarketIndex, MarketLocator},
        token::TokenDataTriple,
    },
    axis_helpers::TokenPair,
    goblin_error::GoblinError,
    input_processor::{ArgsReader, FixedDecode},
    market::MarketReadables,
};

impl<TP: TokenPair + HardcodedMarketList> MarketLocator<TP> for Hardcoded {
    type Locator = MarketIndex<TP>;

    fn decode_locator(
        reader: &ArgsReader,
        _token_data_triple: &TokenDataTriple,
    ) -> Result<Self::Locator, GoblinError> {
        MarketIndex::try_fixed_decode(reader)
    }

    fn locate_market(locator: &Self::Locator) -> &MarketReadables<TP> {
        &TP::HARDCODED_MARKET_LIST[*locator]
    }
}
