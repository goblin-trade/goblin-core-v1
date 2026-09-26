use core::borrow::Borrow;

use crate::{
    axis::{market::MarketLocator, token::TokenDataTriple},
    axis_helpers::MarketSpec,
    goblin_error::GoblinError,
    market::MarketReadables,
    types::Address,
};

pub struct Readables<'a, MS: MarketSpec> {
    pub msg_sender: &'a Address,
    resolved: MS::Readables,
}

impl<'a, MS: MarketSpec> Readables<'a, MS> {
    pub fn new(
        msg_sender: &'a Address,
        locator: &MS::Locator,
        token_data_triple: &TokenDataTriple,
    ) -> Result<Self, GoblinError> {
        Ok(Self {
            msg_sender,
            resolved: MS::Market::locate_market(locator, token_data_triple)?,
        })
    }

    pub fn market_readables(&self) -> &MarketReadables<MS::Pair> {
        self.resolved.borrow()
    }
}
