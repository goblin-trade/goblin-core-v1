use crate::{
    axis::{market::MarketLocator, token::TokenDataTriple},
    axis_helpers::MarketSpec,
    goblin_error::GoblinError,
    market::MarketReadables,
    types::Address,
};

pub struct Readables<'a, MS: MarketSpec> {
    pub msg_sender: &'a Address,
    pub market_readables: MarketReadables<MS::Pair>,
}

impl<'a, MS: MarketSpec> Readables<'a, MS> {
    pub fn new(
        msg_sender: &'a Address,
        locator: &MS::Locator,
        token_data_triple: &TokenDataTriple,
    ) -> Result<Self, GoblinError> {
        Ok(Self {
            msg_sender,
            market_readables: MS::Market::locate_market(locator, token_data_triple)?,
        })
    }
}
