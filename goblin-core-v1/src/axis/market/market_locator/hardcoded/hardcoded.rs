use crate::{
    axis::{
        market::{
            market_locator::{MarketIndex, MarketLocator},
            Hardcoded,
        },
        token::token_reader::TokenDataTriple,
    },
    goblin_error::GoblinError,
    input_processor::{DecodeCtx, FixedDecode},
    market::{token_pair::TokenPair, MarketReadables},
};

impl<TP: TokenPair> MarketLocator for (Hardcoded, TP) {
    type Locator = MarketIndex<Self>;

    fn decode_locator(
        ctx: &DecodeCtx,
        _token_data_triple: &TokenDataTriple,
    ) -> Result<Self::Locator, GoblinError> {
        MarketIndex::try_fixed_decode(ctx)
    }

    fn locate_market(locator: &Self::Locator) -> &MarketReadables<(Hardcoded, TP)> {
        &TP::HARDCODED_MARKET_LIST[*locator]
    }
}
