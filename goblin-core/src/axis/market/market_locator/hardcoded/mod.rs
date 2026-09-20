pub mod hardcoded_market_list;

pub use hardcoded_market_list::*;

use deku::DekuReader;

use crate::{
    axis::{
        market::{Hardcoded, MarketIndex, MarketLocator},
        token::TokenDataTriple,
    },
    axis_helpers::TokenPair,
    goblin_error::GoblinError,
    input_processor::ArgsReaderV2,
    market::MarketReadables,
};

impl<TP: TokenPair + HardcodedMarketList> MarketLocator<TP> for Hardcoded {
    type Locator = MarketIndex<TP>;

    fn decode_locator<'a>(
        reader: &mut ArgsReaderV2<'a>,
        _token_data_triple: &TokenDataTriple,
    ) -> Result<Self::Locator, GoblinError> {
        MarketIndex::<TP>::from_reader_with_ctx(reader, ()).map_err(|_| GoblinError::InvalidPayload)
    }

    fn locate_market(locator: &Self::Locator) -> &MarketReadables<TP> {
        &TP::HARDCODED_MARKET_LIST[*locator]
    }
}
