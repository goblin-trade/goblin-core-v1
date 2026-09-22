use deku::DekuReader;

use crate::{
    axis::{
        market::{Dynamic, HardcodedMarketList, MarketLocator},
        token::TokenDataTriple,
    },
    axis_helpers::TokenPair,
    goblin_error::GoblinError,
    input_processor::ArgsReader,
    market::{CommonMarket, MarketReadables},
    require,
    state::Preimage,
};

impl<TP: TokenPair + HardcodedMarketList> MarketLocator<TP> for Dynamic {
    type Locator = MarketReadables<TP>;

    fn decode_locator<'a>(
        reader: &mut ArgsReader<'a>,
        token_data_triple: &TokenDataTriple,
    ) -> Result<Self::Locator, GoblinError> {
        let common_market = CommonMarket::<TP>::from_reader_with_ctx(reader, ())
            .map_err(|_| GoblinError::InvalidPayload)?;
        require!(
            common_market.lot_size_pair_u32.valid(),
            GoblinError::InvalidLotSize
        );

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
