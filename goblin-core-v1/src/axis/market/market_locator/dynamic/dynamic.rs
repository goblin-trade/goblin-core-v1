use crate::{
    axis::{
        market::{market_locator::MarketLocator, Dynamic},
        token::token_reader::TokenDataTriple,
    },
    goblin_error::GoblinError,
    input_processor::{ArgsReader, FixedDecode},
    market::{token_pair::TokenPair, CommonMarket, MarketReadables},
    state::Preimage,
};

impl<TP: TokenPair> MarketLocator for (Dynamic, TP) {
    type Locator = MarketReadables<Self>;

    fn decode_locator(
        reader: &ArgsReader,
        token_data_triple: &TokenDataTriple,
    ) -> Result<Self::Locator, GoblinError> {
        let common_market = CommonMarket::<(Dynamic, TP)>::try_fixed_decode(reader)?;
        common_market.lot_size_pair.validate()?;

        let preimage = common_market.get_preimage(token_data_triple)?;
        let key = preimage.hash();

        Ok(MarketReadables {
            market: common_market,
            market_key: key,
        })
    }

    fn locate_market(locator: &Self::Locator) -> &MarketReadables<(Dynamic, TP)> {
        locator
    }
}
