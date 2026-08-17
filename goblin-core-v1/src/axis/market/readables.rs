use crate::{axis::market::MarketReadables, axis_helpers::MarketSpec, types::Address};

pub struct Readables<'a, MS: MarketSpec> {
    pub msg_sender: &'a Address,
    pub market_readables: &'a MarketReadables<MS>,
}
