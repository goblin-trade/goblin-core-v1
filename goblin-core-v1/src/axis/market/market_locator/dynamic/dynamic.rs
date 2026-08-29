use crate::{
    axis::{
        market::{
            market_locator::{hardcoded::HardcodedMarketList, MarketLocator},
            Dynamic,
        },
        token::token_reader::TokenDataTriple,
    },
    axis_helpers::TokenPair,
    goblin_error::GoblinError,
    input_processor::{ArgsReader, FixedDecode},
    market::{CommonMarket, MarketReadables},
    state::Preimage,
};

impl<TP: TokenPair + HardcodedMarketList> MarketLocator for (Dynamic, TP) {
    type Locator = MarketReadables<TP>;

    fn decode_locator(
        reader: &ArgsReader,
        token_data_triple: &TokenDataTriple,
    ) -> Result<Self::Locator, GoblinError> {
        let common_market = CommonMarket::<TP>::try_fixed_decode(reader)?;
        common_market.lot_size_pair.validate()?;

        let preimage = common_market.get_preimage(token_data_triple)?;
        let key = preimage.hash();

        Ok(MarketReadables {
            market: common_market,
            market_key: key,
        })
    }

    fn locate_market(locator: &Self::Locator) -> &MarketReadables<TP> {
        locator
    }
}
