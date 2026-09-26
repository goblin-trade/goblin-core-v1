use crate::{
    axis::{
        market::{Dynamic, HardcodedMarketList, MarketLocator},
        token::TokenDataTriple,
    },
    axis_helpers::TokenPair,
    goblin_error::GoblinError,
    market::{CommonMarket, MarketReadables},
    require,
};

impl<TP: TokenPair + HardcodedMarketList> MarketLocator<TP> for Dynamic {
    type Locator = CommonMarket<TP>;

    fn locate_market(
        locator: &Self::Locator,
        token_data_triple: &TokenDataTriple,
    ) -> Result<MarketReadables<TP>, GoblinError> {
        require!(
            locator.lot_size_pair_u32.valid(),
            GoblinError::InvalidLotSize
        );

        // The slot key is derived here, never read from the wire.
        MarketReadables::from_market(*locator, token_data_triple)
    }
}
