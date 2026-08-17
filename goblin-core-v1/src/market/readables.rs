use crate::{axis_helpers::MarketSpec, market::MarketReadables, types::Address};

pub struct Readables<'a, MS: MarketSpec> {
    pub msg_sender: &'a Address,
    pub market_readables: &'a MarketReadables<MS>,
}
