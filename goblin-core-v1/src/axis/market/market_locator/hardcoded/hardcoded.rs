use crate::{
    axis::{
        market::{
            market_locator::{hardcoded::HardcodedMarketList, MarketIndex, MarketLocator},
            Hardcoded,
        },
        token::token_reader::TokenDataTriple,
    },
    axis_helpers::TokenPair,
    goblin_error::GoblinError,
    input_processor::{ArgsReader, FixedDecode},
    market::MarketReadables,
};

impl<TP> MarketLocator for (Hardcoded, TP)
where
    TP: TokenPair + HardcodedMarketList,
{
    type Locator = MarketIndex<TP>;

    fn decode_locator(
        reader: &ArgsReader,
        _token_data_triple: &TokenDataTriple,
    ) -> Result<Self::Locator, GoblinError> {
        MarketIndex::try_fixed_decode(reader)
    }

    fn locate_market(locator: &Self::Locator) -> &MarketReadables<TP>
    where
        TP: HardcodedMarketList,
    {
        &TP::HARDCODED_MARKET_LIST[*locator]
    }
}
