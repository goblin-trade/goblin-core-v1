use crate::{
    axis::{
        market::{
            market_locator::{MarketIndex, MarketLocator},
            Hardcoded,
        },
        token::token_reader::TokenDataTriple,
    },
    axis_helpers::TokenPair,
    goblin_error::GoblinError,
    input_processor::{ArgsReader, FixedDecode},
    market::MarketReadables,
};

impl<TP: TokenPair> MarketLocator for (Hardcoded, TP) {
    type Locator = MarketIndex<Self>;

    fn decode_locator(
        reader: &ArgsReader,
        _token_data_triple: &TokenDataTriple,
    ) -> Result<Self::Locator, GoblinError> {
        MarketIndex::try_fixed_decode(reader)
    }

    fn locate_market(locator: &Self::Locator) -> &MarketReadables<(Hardcoded, TP)> {
        &TP::HARDCODED_MARKET_LIST[*locator]
    }
}
