use crate::{
    axis::market::MarketLocator, axis_helpers::MarketSpec, market::MarketReadables, types::Address,
};

pub struct Readables<'a, MS: MarketSpec> {
    pub msg_sender: &'a Address,
    locator: &'a MS::Locator,
}

impl<'a, MS: MarketSpec> Readables<'a, MS> {
    pub fn new(msg_sender: &'a Address, locator: &'a MS::Locator) -> Self {
        Self {
            msg_sender,
            locator,
        }
    }

    pub fn market_readables(&self) -> &MarketReadables<MS::Pair> {
        MS::Market::locate_market(self.locator)
    }
}
